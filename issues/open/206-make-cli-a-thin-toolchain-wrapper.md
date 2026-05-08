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
completed: 2026-04-28
---

## Summary

Reduce `crates/cli` to command parsing, file/stdout/stderr handling, exit-code mapping, and calls into compiler crates. Compiler pipeline ownership should live outside the CLI crate so tests, playground, editor integrations, and future library consumers can reuse the same API.

## Problem

`crates/cli` still owns compiler responsibilities: frontend parsing, backend emission, dump orchestration, build orchestration, host-deny validation, and command-surface behavior. This keeps implementation pressure in `cli`, makes crate boundaries hard to enforce, and hides reusable compiler APIs behind a binary-oriented crate.

## Desired final state

`crates/cli` is a thin toolchain wrapper. Build and dump commands construct option structs, call a compiler/driver crate, then render success or diagnostics. Frontend, IR, backend, and orchestration logic are owned by their dedicated crates.

## Scope

In scope:

- [x] Add or designate a compiler/driver crate for `build_file`, `build_file_with_options`, `build_file_with_host_deny`, and dump pipeline APIs.
- [x] Move build pipeline orchestration out of `crates/cli/src/lib.rs`.
- [x] Move dump orchestration out of `crates/compiler/src/dump.rs` or make it call reusable compiler APIs only.
- [x] Ensure `crates/cli/src/main.rs` handles command parsing, path conversion, output rendering, and exit codes only.
- [x] Add architecture checks that prevent backend modules from reappearing under `crates/cli/src/`.
- [x] Add architecture checks that prevent parser/compiler pipeline modules from reappearing under `crates/cli/src/`.
- [x] Keep CLI command behavior stable during migration.

Out of scope:

- Implementing new language semantics.
- Implementing typed IR or optimizer dump output; track those in issues 204 and 205.
- Removing frontend code before issue 010 has a usable replacement API.

## Affected paths

Expected:

- `crates/cli/Cargo.toml`
- `crates/cli/src/main.rs`
- `crates/cli/src/lib.rs`
- `crates/compiler/`
- `crates/backend-wasm/`
- `crates/frontend/`
- `crates/ir/`
- `scripts/check/architecture-rules.py`

Do not touch:

- `fixtures/` unless command behavior fixtures need a focused update

## Acceptance criteria

- [x] `crates/cli` no longer defines backend, parser, or compiler pipeline implementation modules.
- [x] CLI build command calls a reusable compiler/driver API.
- [x] CLI dump command calls reusable compiler/driver APIs for each dump phase.
- [x] Architecture checks warn or fail if large compiler implementation files are added under `crates/cli/src/`.
- [x] Existing CLI command tests continue to pass.
- [x] Backend/frontend/IR tests cover moved non-CLI behavior outside `crates/cli`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check-architecture-rules
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

- [x] updated: `docs/04-compiler-architecture-and-runtime.md`
- [x] updated: `docs/12-coding-standard.md`

Current state:

- [x] updated: `current-state.md`

Follow-up issues:

- [x] none

## Notes

Suggested migration order:

1. Finish backend usage migration from the old CLI backend directory to `crates/backend-wasm`.
2. Finish frontend parser extraction tracked by issue 010.
3. Add compiler/driver crate and move build/dump orchestration out of CLI.
4. Tighten architecture checks once the desired boundary exists.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `41b65bd refactor(cli): move compiler driver into compiler crate`
- `3a467f7 chore(checks): keep cli as thin wrapper`
- `6cb3b52 refactor(frontend): move lexer parser out of compiler`
- `c40c672 refactor(backend): split runtime builder modules`
- `8db253f test(cli): tolerate missing official corpora shards`

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run --no-fail-fast
result: PASS (194 passed, 4 skipped)
date: 2026-04-28

command: mise run check-architecture-rules
result: PASS
date: 2026-04-28
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/206-make-cli-a-thin-toolchain-wrapper.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
