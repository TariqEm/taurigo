// Package config is the single place in the sidecar that reads environment
// variables. Nothing outside this package should call os.Getenv directly —
// add a field here instead.
package config

import (
	"os"
	"strconv"
)

// Config holds the sidecar's runtime configuration.
type Config struct {
	// Host is the loopback address the HTTP server binds to. It should stay
	// 127.0.0.1 in normal operation; the field mainly exists so tests and
	// local development can override it (e.g. to bind IPv6 loopback).
	Host string

	// Port is the TCP port to bind. Zero (the default) means "ask the OS for
	// an unused port", which is the production behavior described in
	// CLAUDE.md — never hardcode a port. A non-zero override is only useful
	// for local debugging (e.g. attaching a fixed-port proxy).
	Port int

	// LogLevel controls the verbosity of internal/logging's logger
	// ("debug", "info", "warn", "error"). Defaults to "info".
	LogLevel string

	// JobWorkers is the number of background workers internal/jobs starts.
	// Defaults to a small, sane pool size.
	JobWorkers int
}

const (
	envHost       = "SIDECAR_HOST"
	envPort       = "SIDECAR_PORT"
	envLogLevel   = "SIDECAR_LOG_LEVEL"
	envJobWorkers = "SIDECAR_JOB_WORKERS"

	defaultHost       = "127.0.0.1"
	defaultLogLevel   = "info"
	defaultJobWorkers = 4
)

// Load builds a Config from environment variables, falling back to
// production-sane defaults (OS-assigned port, loopback-only host) when a
// variable is unset or invalid.
func Load() Config {
	cfg := Config{
		Host:       defaultHost,
		Port:       0,
		LogLevel:   defaultLogLevel,
		JobWorkers: defaultJobWorkers,
	}

	if v := os.Getenv(envHost); v != "" {
		cfg.Host = v
	}

	if v := os.Getenv(envPort); v != "" {
		if port, err := strconv.Atoi(v); err == nil && port >= 0 {
			cfg.Port = port
		}
	}

	if v := os.Getenv(envLogLevel); v != "" {
		cfg.LogLevel = v
	}

	if v := os.Getenv(envJobWorkers); v != "" {
		if n, err := strconv.Atoi(v); err == nil && n > 0 {
			cfg.JobWorkers = n
		}
	}

	return cfg
}
