// Command sidecar is the Go backend process Tauri spawns alongside the
// desktop shell. It binds an OS-assigned loopback port, reports that port
// back to its parent (Rust) process via a single stdout handshake line, and
// serves the HTTP API defined in internal/api until it receives a shutdown
// signal.
package main

import (
	"context"
	"errors"
	"fmt"
	"net"
	"net/http"
	"os"
	"os/signal"
	"runtime"
	"syscall"
	"time"

	"github.com/TariqEm/taurigo/apps/sidecar/internal/api"
	"github.com/TariqEm/taurigo/apps/sidecar/internal/config"
	"github.com/TariqEm/taurigo/apps/sidecar/internal/jobs"
	"github.com/TariqEm/taurigo/apps/sidecar/internal/logging"
)

// version and commit are intended to be overridden at build time via
// -ldflags "-X main.version=... -X main.commit=...". They default to
// "dev"/"unknown" for local `go run`/`go build` invocations.
var (
	version = "dev"
	commit  = "unknown"
	builtAt = "unknown"
)

// handshakePrefix is the exact prefix Rust's sidecar-management code looks
// for on the child process's stdout. The full line written is
// "PORT=<n>\n" — keep this in sync with src-tauri/src/sidecar/**.
const handshakePrefix = "PORT="

func main() {
	cfg := config.Load()
	logger := logging.New(cfg.LogLevel)

	listener, err := bindListener(cfg)
	if err != nil {
		logger.Error("failed to bind listener", "error", err)
		os.Exit(1)
	}
	defer listener.Close()

	port, err := listenerPort(listener)
	if err != nil {
		logger.Error("failed to determine bound port", "error", err)
		os.Exit(1)
	}

	// Handshake: emit exactly one line on stdout so the Rust parent process
	// can discover the port. Nothing else should be written to stdout by
	// this process — logs go to stderr (see internal/logging).
	fmt.Println(handshakeLine(port))

	pool := jobs.NewPool(cfg.JobWorkers, logger)

	info := api.BuildInfo{
		Version:   version,
		Commit:    commit,
		BuiltAt:   builtAt,
		GoVersion: runtime.Version(),
	}
	handler := api.NewRouter(logger, info)

	srv := &http.Server{Handler: handler}

	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()

	pool.Start(ctx)

	serveErr := make(chan error, 1)
	go func() {
		serveErr <- srv.Serve(listener)
	}()

	logger.Info("sidecar listening", "addr", listener.Addr().String(), "port", port)

	select {
	case <-ctx.Done():
		logger.Info("shutdown signal received")
	case err := <-serveErr:
		if err != nil && !errors.Is(err, http.ErrServerClosed) {
			logger.Error("server error", "error", err)
		}
	}

	shutdownCtx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := srv.Shutdown(shutdownCtx); err != nil {
		logger.Error("graceful shutdown failed", "error", err)
	}

	pool.Stop()
	logger.Info("sidecar stopped")
}

// bindListener opens the TCP listener the HTTP server will serve on. It
// always binds loopback-only; cfg.Port is 0 in production (OS-assigned
// port) and only ever non-zero when a developer explicitly overrides it via
// SIDECAR_PORT for local debugging.
func bindListener(cfg config.Config) (net.Listener, error) {
	addr := net.JoinHostPort(cfg.Host, fmt.Sprintf("%d", cfg.Port))
	return net.Listen("tcp", addr)
}

// listenerPort extracts the bound TCP port from a listener returned by
// bindListener.
func listenerPort(l net.Listener) (int, error) {
	tcpAddr, ok := l.Addr().(*net.TCPAddr)
	if !ok {
		return 0, fmt.Errorf("listener address is not a TCP address: %v", l.Addr())
	}
	return tcpAddr.Port, nil
}

// handshakeLine formats the stdout line Rust reads to discover the sidecar's
// port, e.g. "PORT=54213".
func handshakeLine(port int) string {
	return fmt.Sprintf("%s%d", handshakePrefix, port)
}
