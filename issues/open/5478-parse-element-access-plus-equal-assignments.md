---
id: 5478
title: "Parse element-access += assignments"
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

Parse arithmetic compound assignment where the target is an element-access
expression, such as `e['hello'] += 1`.

## Problem

`noImplicitAnyStringIndexerOnObject.ts` parses ordinary element access and
simple element assignment, then stops at the first `+=` on an element-access
target:

```text
UnsupportedSyntax: expected Semicolon, got Some(PlusEqual) at 423..425
```

Problem: element-access `+=` is not represented as a complete assignment
expression in the frontend parser.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts
```

Current failing source:

```ts
e['hello'] += 1;
```

Compiler evidence:

```text
tokens: ok; PlusEqual token is present after the element-access target
ast/resolved: fail with expected Semicolon, got Some(PlusEqual)
TypeScript oracle: accepts this statement and later reports TS2538 at map[rover]
```

## Desired final state

The parser accepts the focused element-access `+=` expression and preserves the
assignment target span for later semantic diagnostics or lowering boundaries.

## Scope

In scope:

- [ ] Parse `element[index] += expr` as a frontend assignment expression.
- [ ] Preserve the element-access target span.
- [ ] Re-run the representative triage and record any next parser boundary separately.

Out of scope:

- Postfix element-access updates such as `e['hello']++`.
- Property-access compound assignments, tracked by issue 5311.
- Other compound assignment operators.
- Final noImplicitAny/string-indexer semantic diagnostics.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- focused parser or CLI fixture

Do not touch:

- `crates/backend-wasm/`
- unrelated object literal expression parsing

## Acceptance criteria

- [ ] `noImplicitAnyStringIndexerOnObject.ts` no longer reports `expected Semicolon, got Some(PlusEqual)` at `423..425`.
- [ ] A focused parser fixture covers `obj['key'] += 1;`.
- [ ] Existing simple element assignment parsing for `obj['key'] = value;` still passes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(parser)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from
`issues/done/3552-implement-noImplicitAnyStringIndexerOnObject.md`.

Related but not duplicates:

- `issues/open/5311-parse-property-access-arithmetic-compound-assignments.md`
- `issues/open/5164-parse-exponentiation-compound-assignment.md`
- `issues/open/5178-parse-bitwise-compound-assignment-operators.md`

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
