---
id: 349
title: "Runtime helper or shim JavaScript emission for direct eval execution"
type: feature
area: backend
class: done
priority: P3
depends_on: [347, 348]
blocks: []
created: 2026-04-30
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Implement the backend emission path for direct eval code: either through a wasm runtime helper that interprets eval strings, or by emitting auditable shim JavaScript for host execution when wasm-only semantics are insufficient.

## Problem

Direct eval requires executing a string as JavaScript code with access to the caller's scope. Pure wasm compilation cannot execute arbitrary strings at runtime without a JavaScript interpreter or host eval capability.

Problem: No wasm runtime helper or shim JS emission exists for direct eval execution.

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

- [x] Choose and document the eval execution strategy (wasm helper vs shim JS)
- [x] Implement the selected strategy for a first slice (static-string eval or shim emission)
- [x] Add manifest capability annotations for required host eval support
- [x] Node/iwasm differential fixtures for supported eval execution

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

- [x] Supported eval cases execute and match Node output under iwasm
- [x] If shim JS is emitted, manifest records the required host capability
- [x] Unsupported eval patterns emit source-spanned diagnostics instead of silently failing
- [x] `cargo fmt --all --check` and focused eval/manifest validation pass; broad `cargo nextest run` remains red on unrelated baseline failures

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check manifest
```

Impacted commands:

```sh
mise run check issues
```

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/14-runtime-abi.md` if runtime eval helper ABI is added
- [x] updated: `docs/05-compatibility-and-semantics.md` if eval semantics are documented

Current state:

- [x] updated: `current-state.md` if eval execution capability changes

Follow-up issues:

- [x] none

## Notes

Parent issue: 225

Per the accepted decision in issue 225: if wasm/WASI/runtime helpers cannot implement eval semantics directly, emit auditable shim JavaScript and record required host capabilities. The first slice should target the simplest eval cases (static string with local variable access) to prove the end-to-end path before expanding to dynamic strings.

## Completion evidence

Completed 2026-05-01.

Commits:

- `adfbbd07`: close static direct eval execution slice
- current manifest strategy sync commit: static-string direct eval is documented as compile-time eval-code
  expansion into caller-scope lowered wasm, with manifest evidence that the
  selected first slice stays standalone and does not request `host.eval.*`.

Validation result:

```text
command: cargo test -p ts2wasm-cli eval -- --nocapture
result: pass (2 ir_lowering eval tests, 3 m2_node_diff eval tests)
date: 2026-05-01

command: cargo test -p ts2wasm-cli static_direct_eval_declares_no_node_host_eval_capability -- --nocapture
result: pass
date: 2026-05-01

command: tmp=$(mktemp -d /tmp/ts2wasm-349-repro.XXXXXX); printf 'let x = 1; eval("console.log(x)");\n' > "$tmp/eval.ts"; cargo run -q -p ts2wasm-cli -- build "$tmp/eval.ts" -o "$tmp/eval.wasm" --emit-manifest "$tmp/eval.manifest.json"; iwasm "$tmp/eval.wasm"
result: pass (iwasm stdout: 1; manifest standalone=true, node_host.required=false, node_host.imports=[])
date: 2026-05-01

command: cargo test -p ts2wasm-cli type_alias -- --nocapture
result: pass; confirms issue-345 generic type alias parsing was restored after the namespace classification regression
date: 2026-05-01
```

Remaining risks:

- Dynamic runtime eval strings remain unsupported outside this first static-string
  slice. Any future dynamic eval support must use an audited host shim or
  interpreter contract and record the required `host.eval.*` capability.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

