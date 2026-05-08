---
id: 3557
title: "Implement Noimplicitsymboltostring"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as stale. Fresh coverage shows `noImplicitSymbolToString.ts` now builds
successfully, so this generated parser-syntax blocker no longer needs a child
issue.

## Problem

Older reference results showed 1 case failing in directory
`noImplicitSymbolToString` with diagnostics `parser-syntax`. Fresh evidence on
2026-05-08 shows the file is a build pass:

```text
reference/typescript/tests/cases/compiler/noImplicitSymbolToString.ts: build_pass
```

Problem: no current compiler blocker remains for this generated bucket.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitSymbolToString.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitSymbolToString.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=1 unsupported=0 blocked=0
triage noImplicitSymbolToString.ts: BuildPass
```

## Desired final state

This generated bucket is closed as stale. Do not implement directly from this
bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm no current parser-syntax blocker remains
- [x] Close the stale generated bucket without creating a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is closed stale
- [x] This done issue contains the exact focused `reference-triage` command
- [x] Evidence includes affected path, build-pass status, visible symbols, and parser/TypeScript AST evidence
- [x] No child issue was created because there is no current parser-syntax blocker

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitSymbolToString.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitSymbolToString.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only.
- `cargo nextest run`; issue metadata only.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitSymbolToString.ts`

## Duplicate detection

- none needed; the representative file is a current build pass.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Build pass: noImplicitSymbolToString

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/noImplicitSymbolToString.ts
```

Compiler evidence:

```text
tokens: ok; includes definite-assignment `!`, template literals, `+=`, generic functions, and unary/binary symbol coercion expressions
ast: ok; template literals lower to string concatenation expressions
resolved: ok; global bindings, generic function declarations, local `s1`/`s2` bindings, and coercion expressions resolve
```

TypeScript oracle notes:

```text
diagnostics: TS2731 implicit symbol-to-string conversion and TS2469 symbol `+`/`+=` operator errors
```

The remaining TypeScript oracle diagnostics are semantic parity evidence, not a
current compiler unsupported/parser-syntax blocker in this generated issue.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitSymbolToString.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitSymbolToString.ts
result: pass; BuildPass
date: 2026-05-08
```

Remaining risks:

- Semantic parity for TS2731/TS2469 may need later tracking when the TSC suite
  runs with semantic comparison enabled, but it is not a current
  unsupported/parser-syntax blocker.
