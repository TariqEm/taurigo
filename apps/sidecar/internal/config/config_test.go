package config

import "testing"

func TestLoadDefaults(t *testing.T) {
	t.Setenv(envHost, "")
	t.Setenv(envPort, "")
	t.Setenv(envLogLevel, "")
	t.Setenv(envJobWorkers, "")

	cfg := Load()

	if cfg.Host != defaultHost {
		t.Errorf("expected default host %q, got %q", defaultHost, cfg.Host)
	}
	if cfg.Port != 0 {
		t.Errorf("expected default port 0 (OS-assigned), got %d", cfg.Port)
	}
	if cfg.LogLevel != defaultLogLevel {
		t.Errorf("expected default log level %q, got %q", defaultLogLevel, cfg.LogLevel)
	}
	if cfg.JobWorkers != defaultJobWorkers {
		t.Errorf("expected default job workers %d, got %d", defaultJobWorkers, cfg.JobWorkers)
	}
}

func TestLoadOverridesFromEnv(t *testing.T) {
	t.Setenv(envHost, "0.0.0.0")
	t.Setenv(envPort, "9999")
	t.Setenv(envLogLevel, "debug")
	t.Setenv(envJobWorkers, "8")

	cfg := Load()

	if cfg.Host != "0.0.0.0" {
		t.Errorf("expected overridden host, got %q", cfg.Host)
	}
	if cfg.Port != 9999 {
		t.Errorf("expected overridden port 9999, got %d", cfg.Port)
	}
	if cfg.LogLevel != "debug" {
		t.Errorf("expected overridden log level, got %q", cfg.LogLevel)
	}
	if cfg.JobWorkers != 8 {
		t.Errorf("expected overridden job workers 8, got %d", cfg.JobWorkers)
	}
}

func TestLoadIgnoresInvalidNumericEnv(t *testing.T) {
	t.Setenv(envPort, "not-a-number")
	t.Setenv(envJobWorkers, "not-a-number")

	cfg := Load()

	if cfg.Port != 0 {
		t.Errorf("expected invalid SIDECAR_PORT to fall back to 0, got %d", cfg.Port)
	}
	if cfg.JobWorkers != defaultJobWorkers {
		t.Errorf("expected invalid SIDECAR_JOB_WORKERS to fall back to default, got %d", cfg.JobWorkers)
	}
}
