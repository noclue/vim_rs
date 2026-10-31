"""SHA-256 of a work tree, excluding solver-owned and build files."""

from __future__ import annotations

import hashlib
import os
import sys

# Cargo.lock is rewritten by `cargo build` inside the container. The path
# dependency resolves to /vim_rs there and to the repo crate on the host, so
# the lockfile is not an agent edit.
SKIP = {"src/solve.rs", "src/problem_emit.rs", "Cargo.lock"}


def main() -> None:
    root = sys.argv[1]
    digest = hashlib.sha256()
    for dirpath, _, files in sorted(os.walk(root)):
        for name in sorted(files):
            full = os.path.join(dirpath, name)
            rel = os.path.relpath(full, root).replace("\\", "/")
            if rel in SKIP or rel.startswith("target/"):
                continue
            digest.update(rel.encode())
            with open(full, "rb") as handle:
                digest.update(handle.read())
    print(digest.hexdigest())


if __name__ == "__main__":
    main()
