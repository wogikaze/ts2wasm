---
id: 5445
title: "Parse braced switch case clause statements"
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

Parse braced statement blocks immediately after `case` clauses, such as
`case x.kind === "a": { x.aProps; break; }`.

Split from generated bucket `issues/done/3439-implement-narrowByClauseExpressionInSwitchTrue-parser-syntax.md`.

## Problem

`narrowByClauseExpressionInSwitchTrue6.ts` tokenizes the source, but AST construction stops inside a braced case-clause block when parsing `x.aProps;`. TypeScript accepts the block and later reports unrelated property diagnostics.

Problem: switch case clause parsing does not correctly dispatch a braced block
body containing expression statements with member access.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue6.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected Comma, got Some(Dot) at 670..671
```

Focused coverage:

`reference-coverage` result: `executed=1`, `unsupported=1`, `unsupported_diagcodes=UnsupportedSyntax:1`.

Source context:

```ts
switch (true) {
    default:
        const never: never = x;
    case x.kind === "a": {
        x.aProps;
        break;
    }
    case x.kind === "b": {
        x.bProps;
        break;
    }
}
```

Compiler evidence: tokens are ok through the braced case body, `x.aProps`, and `break`; AST fails while parsing `x.aProps;`; resolved has the same parser failure. TypeScript parses the braced `CaseClause` block and later reports TS2339 property diagnostics.

## Desired final state

The parser accepts braced blocks as switch case clause statements and parses
member-access expression statements inside them. The representative path should
advance beyond `x.aProps;` in the braced case body.

## Scope

In scope:

- [ ] Parse `{ ... }` as a block statement inside switch case/default clause bodies.
- [ ] Preserve existing fallthrough and non-braced case body parsing.
- [ ] Add focused parser coverage for `case pred: { obj.prop; break; }`.
- [ ] Re-run the representative reference path and record the next blocker.

Out of scope: TypeScript control-flow narrowing, TS2339 property diagnostics, class-value `instanceof` runtime support, and ambient declaration name resolution.

## Affected paths

Expected:

- `crates/frontend/src/`
- focused parser tests or fixtures

Do not touch: runtime/backend code unless fresh implementation evidence proves the parser fix exposes a backend-only blocker.

## Acceptance criteria

- [ ] `narrowByClauseExpressionInSwitchTrue6.ts` no longer reports `expected Comma, got Some(Dot)` at `x.aProps;`.
- [ ] A focused parser test covers a braced `case` body containing member
      access and `break`.
- [ ] Existing switch fallthrough fixtures still parse.
- [ ] Any later semantic/type diagnostic from the representative path is
      recorded here or split into a follow-up issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(parser) or test(switch)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue6.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByClauseExpressionInSwitchTrue6.ts --detail --no-dashboard-data
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

Related paths from the same generated bucket are owned by issue 5161 (`declare const f`) and issue 5192 (`instanceof Derived1` class value).

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
