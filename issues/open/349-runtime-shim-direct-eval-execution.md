---
id: 349
title: "Runtime helper or shim JavaScript emission for direct eval execution"
type: feature
area: backend
class: implementation-ready
priority: P3
depends_on: [347, 348]
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Implement the backend emission path for direct eval code: either through a wasm runtime helper that interprets eval strings, or by emitting auditable shim JavaScript for host execution when wasm-only semantics are insufficient.

## Problem

Direct eval requires executing a string as JavaScript code with access to the caller's scope. Pure wasm compilation cannot execute arbitrary strings at runtime without a JavaScript interpreter or host eval capability.

## Current failure

```sh
tmp=/tmp/ts2wasm-349-eval-execution.ts
printf 'let x = 1; eval("console.log(x)");\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-349-eval-execution.wasm
iwasm /tmp/ts2wasm-349-eval-execution.wasm
```

Current result: eval code is not executable at runtime; either the build fails or the wasm module has no eval execution path.

## Desired final state

Direct eval code executes with caller-local scope access. The implementation strategy is chosen per slice:
- For small static-string eval: inline lowering to wasm when parser can prove the eval string
- For dynamic-string eval: emit shim JavaScript with documented required host capabilities
- The manifest/link planning reflects any required host eval capability

## Scope

In scope:

- [ ] Choose and document the eval execution strategy (wasm helper vs shim JS)
- [ ] Implement the selected strategy for a first slice (static-string eval or shim emission)
- [ ] Add manifest capability annotations for required host eval support
- [ ] Node/iwasm differential fixtures for supported eval execution

Out of scope:

- Parser/resolver eval detection (issue 347)
- IR lowering of eval code (issue 348)
- Full indirect eval or global-environment manipulation

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/cli/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/eval*`
- `docs/14-runtime-abi.md`

Do not touch:

- `crates/frontend/src/`
- `crates/ir/src/lowered/`

## Acceptance criteria

- [ ] Supported eval cases execute and match Node output under iwasm
- [ ] If shim JS is emitted, manifest records the required host capability
- [ ] Unsupported eval patterns emit source-spanned diagnostics instead of silently failing
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check manifest
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --limit 300
cargo test -p ts2wasm-cli eval
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/14-runtime-abi.md` if runtime eval helper ABI is added
- [ ] updated: `docs/05-compatibility-and-semantics.md` if eval semantics are documented

Current state:

- [ ] updated: `current-state.md` if eval execution capability changes

Follow-up issues:

- [ ] none

## Notes

Parent issue: 225

Per the accepted decision in issue 225: if wasm/WASI/runtime helpers cannot implement eval semantics directly, emit auditable shim JavaScript and record required host capabilities. The first slice should target the simplest eval cases (static string with local variable access) to prove the end-to-end path before expanding to dynamic strings.

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

- Host eval capability may not be available in all wasm runtimes (e.g., pure WASI without JS host)
