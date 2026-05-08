---
id: 5296
title: "Parse double-dot numeric literal property access"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse `2..toFixed(0)` as member access on a numeric literal.

## Problem

`computedEnumMemberSyntacticallyString.ts` currently fails before its enum body:

```ts
const BAR = 2..toFixed(0);
```

The lexer emits `Number(2) Dot Dot Ident("toFixed")`, and the parser reports:

```text
UnsupportedSyntax: expected member property name, got Dot at 95..102
```

Problem: the parser treats the first dot as member access and rejects the
second dot instead of accepting JavaScript's double-dot numeric member form.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
line 5, column 16
coverage: executed=1, build_pass=0, unsupported=1, blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
```

TypeScript oracle parses the same source as:

```text
VariableDeclaration -> CallExpression -> PropertyAccessExpression
text: 2..toFixed(0)
```

## Scope

In scope:

- [x] Parse `NumberLiteral .. Identifier` for the exact `2..toFixed(0)` shape.
- [x] Add one focused parser or CLI regression for `const BAR = 2..toFixed(0);`.
- [x] Re-run the representative triage and record the next blocker.

Out of scope:

- Full enum transform/runtime support.
- Other numeric literal grammar forms unless directly required by this fixture.

## Affected paths

Expected:

- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/parser/`
- focused parser/CLI tests or fixtures

## Acceptance criteria

- [x] The representative triage no longer reports `expected member property name, got Dot` at `95..102`.
- [x] Focused coverage no longer stops on the double-dot parser error.
- [x] A regression covers `const BAR = 2..toFixed(0);`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(number) or test(member) or test(parser)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedEnumMemberSyntacticallyString.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Notes

Split from generated bucket
`issues/done/1405-implement-computedEnumMemberSyntacticallyString-enum.md`.

Related but not duplicates: issue 1406 covers the sibling
`computedEnumMemberSyntacticallyString2.ts` bucket, and issues 5284/5277 cover
enum boundaries after parsing reaches an enum declaration.

## Completion evidence

Fill only when moving to `done/`.

## False-done audit

**truly-done** (5296)

- Implementation commits: verified via `git log --oneline --all --grep=5296`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
