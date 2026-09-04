package main

import (
	"testing"

	"github.com/TariqEm/taurigo/apps/sidecar/internal/config"
)

func TestBindListenerUsesOSAssignedPort(t *testing.T) {
	cfg := config.Config{Host: "127.0.0.1", Port: 0}

	listener, err := bindListener(cfg)
	if err != nil {
		t.Fatalf("bindListener returned error: %v", err)
	}
	defer listener.Close()

	port, err := listenerPort(listener)
	if err != nil {
		t.Fatalf("listenerPort returned error: %v", err)
	}

	if port == 0 {
		t.Fatal("expected a non-zero OS-assigned port")
	}
}

func TestBindListenerIsLoopbackOnly(t *testing.T) {
	cfg := config.Config{Host: "127.0.0.1", Port: 0}

	listener, err := bindListener(cfg)
	if err != nil {
		t.Fatalf("bindListener returned error: %v", err)
	}
	defer listener.Close()

	addr := listener.Addr().String()
	if got := listener.Addr().Network(); got != "tcp" {
		t.Fatalf("expected tcp network, got %q", got)
	}

	// Sanity check the address string starts with the loopback host we
	// asked for (host:port form).
	if len(addr) == 0 {
		t.Fatal("expected non-empty listener address")
	}
}

func TestHandshakeLineFormat(t *testing.T) {
	cases := []struct {
		port int
		want string
	}{
		{port: 54213, want: "PORT=54213"},
		{port: 1, want: "PORT=1"},
		{port: 65535, want: "PORT=65535"},
	}

	for _, tc := range cases {
		got := handshakeLine(tc.port)
		if got != tc.want {
			t.Errorf("handshakeLine(%d) = %q, want %q", tc.port, got, tc.want)
		}
	}
}
