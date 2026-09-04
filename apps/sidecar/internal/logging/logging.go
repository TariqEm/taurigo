// Package logging is the single place in the sidecar that configures
// structured logging. Nothing outside this package should call slog.SetDefault
// or otherwise construct loggers from scratch.
package logging

import (
	"log/slog"
	"os"
	"strings"
)

// New builds a structured (JSON) slog.Logger writing to stderr, so that
// stdout stays reserved for the port handshake line the Rust parent process
// reads.
//
// levelName is parsed case-insensitively ("debug", "info", "warn", "error");
// an unrecognized or empty value falls back to "info".
func New(levelName string) *slog.Logger {
	handler := slog.NewJSONHandler(os.Stderr, &slog.HandlerOptions{
		Level: parseLevel(levelName),
	})
	return slog.New(handler)
}

func parseLevel(levelName string) slog.Level {
	switch strings.ToLower(strings.TrimSpace(levelName)) {
	case "debug":
		return slog.LevelDebug
	case "warn", "warning":
		return slog.LevelWarn
	case "error":
		return slog.LevelError
	default:
		return slog.LevelInfo
	}
}
