package api

import (
	"encoding/json"
	"net/http"
)

// healthResponse is the body of GET /health.
type healthResponse struct {
	Status string `json:"status"`
}

func handleHealth(w http.ResponseWriter, _ *http.Request) {
	writeJSON(w, http.StatusOK, healthResponse{Status: "ok"})
}

// handleVersion returns a handler for GET /version that reports the
// sidecar's build info. This is the "one real domain endpoint" for Phase 7 —
// a minimal, genuinely useful proof that Rust can round-trip a request to
// the Go sidecar and get back structured data, without any real product
// logic yet.
func handleVersion(info BuildInfo) http.HandlerFunc {
	return func(w http.ResponseWriter, _ *http.Request) {
		writeJSON(w, http.StatusOK, info)
	}
}

// writeJSON writes v as a JSON response body with the given status code.
func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(v)
}
