// Package jobs provides a small worker-pool skeleton for background work
// (file indexing, watching, sync, etc.). API handlers should enqueue a Job
// here rather than doing long-running work inline on the request goroutine.
//
// No concrete job types are defined yet — this is intentionally just the
// pool lifecycle, ready for real job types to be added as features land.
package jobs

import (
	"context"
	"log/slog"
	"sync"
)

// Job is a unit of background work. Run receives a context that is canceled
// when the pool is stopped, so long-running jobs should respect ctx.Done().
type Job interface {
	Run(ctx context.Context) error
}

// JobFunc adapts a plain function to the Job interface.
type JobFunc func(ctx context.Context) error

// Run implements Job.
func (f JobFunc) Run(ctx context.Context) error { return f(ctx) }

// Pool is a fixed-size pool of workers pulling Jobs off a shared queue.
// The zero value is not usable; construct one with NewPool.
type Pool struct {
	logger  *slog.Logger
	workers int
	queue   chan Job

	mu      sync.Mutex
	running bool
	cancel  context.CancelFunc
	wg      sync.WaitGroup
}

// defaultQueueSize is the buffer size of the job queue channel. It's large
// enough to absorb short bursts without callers blocking, without being
// unbounded.
const defaultQueueSize = 256

// NewPool constructs a Pool with the given number of workers. workers is
// clamped to at least 1. logger must not be nil; pass a logger built by
// internal/logging.
func NewPool(workers int, logger *slog.Logger) *Pool {
	if workers < 1 {
		workers = 1
	}
	return &Pool{
		logger:  logger,
		workers: workers,
		queue:   make(chan Job, defaultQueueSize),
	}
}

// Start launches the worker goroutines. It is safe to call once; calling it
// again while already running is a no-op.
func (p *Pool) Start(ctx context.Context) {
	p.mu.Lock()
	defer p.mu.Unlock()

	if p.running {
		return
	}

	runCtx, cancel := context.WithCancel(ctx)
	p.cancel = cancel
	p.running = true

	for i := 0; i < p.workers; i++ {
		p.wg.Add(1)
		go p.worker(runCtx, i)
	}

	p.logger.Info("job pool started", "workers", p.workers)
}

// Stop signals all workers to finish their current job and exit, then waits
// for them to do so. It is safe to call even if the pool was never started.
func (p *Pool) Stop() {
	p.mu.Lock()
	if !p.running {
		p.mu.Unlock()
		return
	}
	cancel := p.cancel
	p.running = false
	p.mu.Unlock()

	cancel()
	p.wg.Wait()
	p.logger.Info("job pool stopped")
}

// Submit enqueues a job for a worker to pick up. It returns false, without
// blocking, if the queue is full — callers should treat that as backpressure
// (e.g. respond 503 from an API handler) rather than block the request
// goroutine.
func (p *Pool) Submit(job Job) bool {
	select {
	case p.queue <- job:
		return true
	default:
		return false
	}
}

func (p *Pool) worker(ctx context.Context, id int) {
	defer p.wg.Done()

	for {
		select {
		case <-ctx.Done():
			return
		case job := <-p.queue:
			if err := job.Run(ctx); err != nil {
				p.logger.Error("job failed", "worker", id, "error", err)
			}
		}
	}
}
