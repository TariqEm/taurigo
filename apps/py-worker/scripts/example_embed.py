"""Example dev-only script — template for future ML/data scripts.

Track A (see BUILD_TIMELINE.md Phase 8): this runs on a developer's machine via
`uv run`, is never bundled or shipped with the desktop app, and never becomes a
sidecar process. If a feature later needs Python at runtime in the shipped app,
that's Phase 8 Track B (a packaged Python sidecar), not this.

Usage:
    uv run apps/py-worker/scripts/example_embed.py "some text"
"""

import json
import sys


def fake_embed(text: str) -> list[float]:
    # Placeholder: replace with a real embedding model when a feature needs one.
    return [round(len(word) / 10, 3) for word in text.split()]


def main() -> None:
    text = " ".join(sys.argv[1:]) or "hello from py-worker"
    print(json.dumps({"text": text, "embedding": fake_embed(text)}))


if __name__ == "__main__":
    main()
