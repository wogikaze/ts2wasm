---
id: 3440
title: "Implement Narrowbyclauseexpressioninswitchtrue Unknown Unsupported"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed after splitting the current parser blocker to
`issues/open/5446-parse-empty-statements-in-switch-case-bodies.md`.

## Problem

Reference test results show 1 case fails in directory
`narrowByClauseExpressionInSwitchTrue-unknown-unsupported` with diagnostics:
unknown-unsupported. Fresh triage shows this is an ordinary empty statement
parser blocker from `cond2;;`.

Problem: narrowByClauseExpressionInSwitchTrue-unknown-unsupported had 1
generated reference failure and needed smart-triage evidence before
implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
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
- [x] Child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5446-parse-empty-statements-in-switch-case-bodies.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts`

## Duplicate detection

- `issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md` - Implement Arraytolocalestringes Unknown Unsupported (same feature label, title overlap)
- `issues/done/5282-parse-labeled-empty-statements.md` is related but
  distinct; it owns labeled empty statements, not ordinary `;;` inside switch
  case bodies.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Fresh triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts

result:
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 353, end: 354 } }) at 395..400
```

Source context:

```ts
case cond2:
    cond1; // false
    cond2;; // never
    break;
```

Compiler evidence:

```text
tokens: ok; includes two consecutive Semicolon tokens after cond2
ast: fails on the second Semicolon
resolved: same parser failure
TypeScript oracle: ok, diagnostics=[]
```

## Completion evidence

Closed after splitting issue 5446.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts
result: pass; split to issue 5446 for empty statements in switch case bodies
date: 2026-05-08
```

Remaining risks:

- Later `switch (true)` narrowing semantics may need a focused follow-up after
  the parser advances.
