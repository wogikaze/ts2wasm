---
id: 5446
title: "Parse empty statements in switch case bodies"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Parse empty statements produced by extra semicolons inside switch case bodies,
such as `cond2;;`.

Split from generated bucket
`issues/open/3440-implement-narrowByClauseExpressionInSwitchTrue-unknown-unsupported.md`.

## Problem

`narrowByClauseExpressionInSwitchTrue10.ts` tokenizes `cond2;;` as an
expression statement followed by a standalone semicolon. The parser then tries
to parse that standalone semicolon as an expression and reports unsupported
syntax before reaching the following `break`.

Problem: ordinary empty statements in switch case bodies are not accepted as
no-op statements.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts
```

Current diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Semicolon, span: Span { start: 353, end: 354 } }) at 395..400
```

Focused coverage:

```text
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
case cond2:
    cond1; // false
    cond2;; // never
    break;
```

Compiler evidence: tokens are ok and include two consecutive semicolons after
`cond2`; AST fails on the second semicolon; TypeScript parses the file with no
diagnostics.

## Desired final state

The parser accepts standalone semicolons as empty no-op statements inside switch
case bodies, and the representative path advances past `cond2;;`.

## Scope

In scope:

- [ ] Parse a standalone semicolon as an empty statement in switch case/default
      bodies.
- [ ] Preserve existing expression statement parsing for the preceding
      expression before the extra semicolon.
- [ ] Add focused parser coverage for `switch (true) { case c: x;; break; }`.
- [ ] Re-triage the representative path and record the next blocker if needed.

Out of scope:

- Labeled empty statements, tracked separately by issue 5282.
- TypeScript control-flow narrowing for `switch (true)`.
- General ASI refactors outside this empty-statement case.

## Affected paths

Expected:

- `crates/frontend/src/`
- focused parser tests or fixtures

Do not touch:

- runtime/backend code unless fresh implementation evidence proves the parser
  fix exposes a backend-only blocker

## Acceptance criteria

- [ ] `narrowByClauseExpressionInSwitchTrue10.ts` no longer reports an
      unsupported expression for the second semicolon in `cond2;;`.
- [ ] A focused parser test covers an empty statement in a switch case body.
- [ ] Existing switch case and break parsing tests still pass.
- [ ] Any later semantic/narrowing blocker is recorded here or split to a
      follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(parser) or test(switch)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue10.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Related but distinct issue: `issues/open/5282-parse-labeled-empty-statements.md`
owns `Input: ;`; this issue owns ordinary unlabeled empty statements in switch
case bodies.

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
