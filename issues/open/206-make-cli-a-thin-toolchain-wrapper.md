---
id: 206
title: "Make CLI a thin toolchain wrapper"
type: refactor
area: cli
class: implementation-ready
priority: P1
depends_on: [010, 026]
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Reduce `crates/cli` to command parsing, file/stdout/stderr handling, exit-code mapping, and calls into compiler crates. Compiler pipeline ownership should live outside the CLI crate so tests, playground, editor integrations, and future library consumers can reuse the same API.

## Problem

`crates/cli` still owns compiler responsibilities: frontend parsing, backend emission, dump orchestration, build orchestration, host-deny validation, and command-surface behavior. This keeps implementation pressure in `cli`, makes crate boundaries hard to enforce, and hides reusable compiler APIs behind a binary-oriented crate.

## Desired final state

`crates/cli` is a thin toolchain wrapper. Build and dump commands construct option structs, call a compiler/driver crate, then render success or diagnostics. Frontend, IR, backend, and orchestration logic are owned by their dedicated crates.

## Scope

In scope:

- [ ] Add or designate a compiler/driver crate for `build_file`, `build_file_with_options`, `build_file_with_host_deny`, and dump pipeline APIs.
- [ ] Move build pipeline orchestration out of `crates/cli/src/lib.rs`.
- [ ] Move dump orchestration out of `crates/cli/src/dump.rs` or make it call reusable compiler APIs only.
- [ ] Ensure `crates/cli/src/main.rs` handles command parsing, path conversion, output rendering, and exit codes only.
- [ ] Add architecture checks that prevent backend/parser/compiler pipeline modules from reappearing under `crates/cli/src/`.
- [ ] Keep CLI command behavior stable during migration.

Out of scope:

- Implementing new language semantics.
- Implementing typed IR or optimizer dump output; track those in issues 204 and 205.
- Removing frontend/backend code before issues 010 and 026 have usable replacement APIs.

## Affected paths

Expected:

- `crates/cli/Cargo.toml`
- `crates/cli/src/main.rs`
- `crates/cli/src/lib.rs`
- `crates/cli/src/dump.rs`
- `crates/backend-wasm/`
- `crates/frontend/`
- `crates/ir/`
- future compiler or driver crate path, once the crate is created
- `scripts/check/architecture-rules.py`

Do not touch:

- `fixtures/` unless command behavior fixtures need a focused update

## Acceptance criteria

- [ ] `crates/cli` no longer defines backend, parser, or compiler pipeline implementation modules.
- [ ] CLI build command calls a reusable compiler/driver API.
- [ ] CLI dump command calls reusable compiler/driver APIs for each dump phase.
- [ ] Architecture checks warn or fail if large compiler implementation files are added under `crates/cli/src/`.
- [ ] Existing CLI command tests continue to pass.
- [ ] Backend/frontend/IR tests cover moved non-CLI behavior outside `crates/cli`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
scripts/manager check-architecture-rules
```

Impacted commands:

```sh
cargo check -p ts2wasm-cli
cargo check -p ts2wasm-backend-wasm
cargo nextest run -p ts2wasm-cli
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/04-compiler-architecture-and-runtime.md`
- [ ] updated: `docs/12-coding-standard.md`

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Suggested migration order:

1. Finish backend usage migration from `crates/cli/src/backend` to `crates/backend-wasm`.
2. Finish frontend parser extraction tracked by issue 010.
3. Add compiler/driver crate and move build/dump orchestration out of CLI.
4. Tighten architecture checks once the desired boundary exists.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
