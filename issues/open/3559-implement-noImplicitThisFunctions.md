---
id: 3559
title: "Implement Noimplicitthisfunctions"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5273]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as a generated bucket. Fresh evidence shows the current blocker is the
existing nested zero-argument arrow parser issue 5273.

## Problem

Fresh triage reaches `let f5 = () => () => this;`, then AST construction
rejects the `RightParen` immediately before the inner arrow:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 455, end: 456 } }) at 457..459
```

Problem: this generated bucket is superseded by issue 5273, which owns nested
arrow expression bodies such as `() => () => ...`.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitThisFunctions.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitThisFunctions.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
triage: UnsupportedSyntax at the nested zero-argument arrow in `let f5 = () => () => this;`
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5273-parse-nested-zero-argument-arrow-returns.md`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5273 covers this bucket
- [x] Fold into existing issue 5273 for the same observable parser boundary
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
- [x] Existing issue 5273 contains the implementation owner; this done issue contains the exact focused triage command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5273 acceptance covers the nested zero-argument arrow parser failure

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitThisFunctions.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitThisFunctions.ts
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

- [x] folded into: `issues/open/5273-parse-nested-zero-argument-arrow-returns.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitThisFunctions.ts`

## Duplicate detection

- `issues/open/5273-parse-nested-zero-argument-arrow-returns.md` is the exact
  owner for the current nested zero-argument arrow parser failure.
- `issues/open/5179-report-implicit-this-before-closure-runtime-guard.md` and
  `issues/open/5218a-support-nested-function-closures-capturing-this.md` may be
  relevant after parsing advances, but they are later semantic/runtime owners,
  not the current first blocker.
- Folded into issue 5273.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage unknown unsupported: noImplicitThisFunctions

- Issue class: triage-needed
- Feature label: unknown-unsupported
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/noImplicitThisFunctions.ts
```

Current compiler message:

```text
unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 455, end: 456 } }) at 457..459
```

Source context:

```ts
let f4: (b: number) => number = b => this.c + b;
let f5 = () => () => this;

let f6 = function() { return () => this; };
let f7 = function() { return function() { return this } };
```

Compiler evidence:

```text
tokens: ok through f1, f2, f3, f4, and f5
ast/resolved: fail at the nested zero-argument arrow in `() => () => this`
visible symbols: f1, f2, f3, f4, f5, partial f6 binding
```

TypeScript oracle:

```text
TS2683 for f3/f6/f7 `this`
TS7041 for arrow functions capturing global `this`
```

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitThisFunctions.ts --detail --no-dashboard-data
result: pass; reproduced current unsupported boundary
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitThisFunctions.ts
result: pass; generated smart triage evidence and duplicate review material
date: 2026-05-08
```

Remaining risks:

- After issue 5273 advances, this fixture may expose implicit-this diagnostic
  parity or nested-function `this` runtime boundaries.
