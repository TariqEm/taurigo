// Package api holds the sidecar's HTTP handlers. Handlers stay thin: they
// validate input, enqueue background work through internal/jobs when it's
// more than trivial, and write a response — they don't block on long-running
// work themselves.
package api

import (
	"log/slog"
	"net/http"
	"time"
)

// BuildInfo describes the running sidecar binary, surfaced via GET /version.
// Fields are populated by cmd/sidecar/main.go, optionally from -ldflags at
// build time.
type BuildInfo struct {
	Version   string `json:"version"`
	Commit    string `json:"commit"`
	BuiltAt   string `json:"builtAt"`
	GoVersion string `json:"goVersion"`
}

// NewRouter builds the sidecar's HTTP handler tree.
//
// It uses stdlib net/http.ServeMux (Go 1.22+ method-aware patterns) rather
// than a third-party router — the sidecar's surface area is small enough
// that a routing library isn't worth the extra dependency.
func NewRouter(logger *slog.Logger, info BuildInfo) http.Handler {
	mux := http.NewServeMux()

	mux.HandleFunc("GET /health", handleHealth)
	mux.HandleFunc("GET /version", handleVersion(info))

	return withLogging(logger, mux)
}

// withLogging wraps a handler with minimal structured request logging.
func withLogging(logger *slog.Logger, next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		rec := &statusRecorder{ResponseWriter: w, status: http.StatusOK}

		next.ServeHTTP(rec, r)

		logger.Info("request",
			"method", r.Method,
			"path", r.URL.Path,
			"status", rec.status,
			"duration_ms", time.Since(start).Milliseconds(),
		)
	})
}

// statusRecorder captures the status code written by a handler so it can be
// logged after the fact.
type statusRecorder struct {
	http.ResponseWriter
	status int
}

func (r *statusRecorder) WriteHeader(status int) {
	r.status = status
	r.ResponseWriter.WriteHeader(status)
}
