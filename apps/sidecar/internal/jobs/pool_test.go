package jobs

import (
	"context"
	"sync/atomic"
	"testing"
	"time"

	"github.com/TariqEm/taurigo/apps/sidecar/internal/logging"
)

func TestPoolRunsSubmittedJobs(t *testing.T) {
	pool := NewPool(2, logging.New("debug"))
	pool.Start(context.Background())
	defer pool.Stop()

	var ran atomic.Int32
	done := make(chan struct{})

	ok := pool.Submit(JobFunc(func(ctx context.Context) error {
		ran.Add(1)
		close(done)
		return nil
	}))
	if !ok {
		t.Fatal("expected Submit to succeed")
	}

	select {
	case <-done:
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for job to run")
	}

	if ran.Load() != 1 {
		t.Fatalf("expected job to run exactly once, ran %d times", ran.Load())
	}
}

func TestPoolStopWaitsForWorkers(t *testing.T) {
	pool := NewPool(1, logging.New("debug"))
	pool.Start(context.Background())

	// Stop should return promptly even with no jobs pending.
	stopped := make(chan struct{})
	go func() {
		pool.Stop()
		close(stopped)
	}()

	select {
	case <-stopped:
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for Stop to return")
	}
}

func TestPoolStartIsIdempotent(t *testing.T) {
	pool := NewPool(1, logging.New("debug"))
	pool.Start(context.Background())
	pool.Start(context.Background()) // should be a no-op, not panic/deadlock
	pool.Stop()
}

func TestPoolStopWithoutStartIsSafe(t *testing.T) {
	pool := NewPool(1, logging.New("debug"))
	pool.Stop() // should not panic
}
