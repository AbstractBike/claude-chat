# Claude Chat — Matrix Agent Platform

Single Rust binary bridging Matrix rooms to Claude CLI sessions with bubblewrap sandboxing,
encrypted secrets via MCP, and full observability.

## Language
All code and docs in English.

## Structure
- `src/main.rs` — entry point, config loading, Matrix sync loop
- `src/config.rs` — TOML configuration
- `src/matrix/` — Matrix client, handler, sender, control commands
- `src/session/` — Claude CLI session manager, agent state persistence
- `src/sandbox/` — Bubblewrap filesystem-only sandboxing
- `src/agent/` — Inter-agent tool call parser
- `src/secrets/` — MCP secrets vault with age encryption
- `src/observability/` — JSON logging, Prometheus metrics
- `tests/` — Rust integration tests
- `flake.nix` — Nix flake with Home Manager module
- `docs/plans/` — Design documents
