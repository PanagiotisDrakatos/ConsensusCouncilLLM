# Install

## Requirements

- Rust toolchain with `cargo`
- a writable build target directory

Tested in this repository with:
- `cargo 1.92.0`
- `rustc 1.92.0`

## Build

Standard build:

```bash
cargo build
```

If the repository lives in a synced or restricted directory and Cargo cannot create `target/`, use:

```bash
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo build
```

## Test

```bash
CARGO_TARGET_DIR=/tmp/consensuscouncil-target cargo test
```

## Notes

- No API keys are required.
- No network provider setup is required.
- The current prototype is file-based and fixture-driven.
