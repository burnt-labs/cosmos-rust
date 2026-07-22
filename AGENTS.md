# CLAUDE.md

## Project Overview

cosmos-rust is a Rust crate collection for Cosmos SDK interactions, containing protobuf definitions and client libraries. This is the burnt-labs fork.

## Tech Stack

- **Language**: Rust
- **Key Crates**: `cosmos-sdk-proto`, `cosmrs`
- **Build**: Cargo workspace

## Development Commands

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Check without building
cargo check

# Format code
cargo fmt

# Lint
cargo clippy

# Regenerate protobuf bindings (requires proto-build)
cargo run --bin proto-build
```

## Workspace Structure

```
cosmos-sdk-proto/    # Protobuf-generated Cosmos SDK types
cosmrs/              # High-level Cosmos SDK client library
proto-build/         # Protobuf code generation tooling
cosmos-sdk-go/       # Go SDK reference files
ibc-go/              # IBC Go reference files
wasmd/               # wasmd reference files
```

## Code Conventions

- Follow standard Rust naming conventions
- Use `cargo fmt` before committing
- All public APIs must have doc comments
- Protobuf bindings are auto-generated - edit the generator not the output
