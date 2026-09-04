package api

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/TariqEm/taurigo/apps/sidecar/internal/logging"
)

func TestHealthEndpoint(t *testing.T) {
	router := NewRouter(logging.New("debug"), BuildInfo{Version: "test"})

	req := httptest.NewRequest(http.MethodGet, "/health", nil)
	rec := httptest.NewRecorder()

	router.ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", rec.Code)
	}

	var body healthResponse
	if err := json.NewDecoder(rec.Body).Decode(&body); err != nil {
		t.Fatalf("failed to decode response body: %v", err)
	}

	if body.Status != "ok" {
		t.Fatalf("expected status %q, got %q", "ok", body.Status)
	}
}

func TestVersionEndpoint(t *testing.T) {
	want := BuildInfo{Version: "1.2.3", Commit: "abc123", BuiltAt: "2026-09-04"}
	router := NewRouter(logging.New("debug"), want)

	req := httptest.NewRequest(http.MethodGet, "/version", nil)
	rec := httptest.NewRecorder()

	router.ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", rec.Code)
	}

	var got BuildInfo
	if err := json.NewDecoder(rec.Body).Decode(&got); err != nil {
		t.Fatalf("failed to decode response body: %v", err)
	}

	if got != want {
		t.Fatalf("expected %+v, got %+v", want, got)
	}
}

func TestUnknownRouteReturns404(t *testing.T) {
	router := NewRouter(logging.New("debug"), BuildInfo{})

	req := httptest.NewRequest(http.MethodGet, "/nope", nil)
	rec := httptest.NewRecorder()

	router.ServeHTTP(rec, req)

	if rec.Code != http.StatusNotFound {
		t.Fatalf("expected status 404, got %d", rec.Code)
	}
}
