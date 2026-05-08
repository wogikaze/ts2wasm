---
id: 3554
title: "Implement Noimplicitreturnsexclusions"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5240]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the current blocker is the
existing async-arrow parser issue 5240.

## Problem

Fresh triage reaches the `registerCommand(..., async (item) => { ... })` call
near the end of the fixture. The parser then rejects the `async` token before
building an arrow-function AST:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 1928, end: 1933 } }) at 1934..1935
```

Problem: this generated bucket is superseded by issue 5240, which owns raw
`unsupported expression: Async` parser failures for async arrow function
expressions.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsExclusions.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitReturnsExclusions.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
triage: UnsupportedSyntax unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 1928, end: 1933 } }) at 1934..1935
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5240-w2-docs-audit-and-stale-entries.md`. Do not implement
directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold into existing issue 5240 for the same observable parser boundary
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing issue 5240 contains the implementation owner; this done issue contains the exact focused triage command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5240 acceptance covers the raw async-arrow parser failure; noImplicitReturns behavior must be re-triaged after 5240 advances

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitReturnsExclusions.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsExclusions.ts
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

- [x] folded into: `issues/open/5240-w2-docs-audit-and-stale-entries.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitReturnsExclusions.ts`

## Duplicate detection

- `issues/open/5240-w2-docs-audit-and-stale-entries.md` is the exact
  owner for `async (item) => { ... }` failing with raw
  `unsupported expression: Async` before AST construction.
- `issues/open/750-implement-asyncFunctionContextuallyTypedReturns.md`,
  `issues/open/751-implement-asyncFunctionNoReturnType.md`, and related older
  generated async buckets are broader/stale generated buckets, while 5240 is
  the narrow implementation-ready parser slice.
- Folded into issue 5240.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage unknown unsupported: noImplicitReturnsExclusions

- Issue class: triage-needed
- Feature label: unknown-unsupported
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/noImplicitReturnsExclusions.ts
```

Failure location:

```text
line 94, column 28
unsupported expression: Some(SpannedToken { kind: Async, span: Span { start: 1928, end: 1933 } }) at 1934..1935
```

Source context:

```ts
registerCommand("_references-view.showHistoryItem", async (item) => {
  if (item instanceof HistoryItem) {
    return executeCommand("vscode.open", item.input.location.uri);
  }
});
```

Compiler evidence:

```text
tokens: ok through prior noImplicitReturns functions, declarations, and the call expression
ast/resolved: fail before async arrow construction with raw Async parser error
visible symbols: f1..f16, HistoryItem, registerCommand
```

TypeScript oracle:

```text
diagnostics: []
```

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitReturnsExclusions.ts --detail --no-dashboard-data
result: pass; reproduced current UnsupportedSyntax unknown-unsupported boundary at async arrow call argument
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitReturnsExclusions.ts
result: pass; generated smart triage evidence and duplicate review material
date: 2026-05-08
```

Remaining risks:

- After issue 5240 advances, this fixture may expose async runtime,
  contextual return type, or `noImplicitReturns` exclusion behavior that needs a
  narrower follow-up issue.
