---
id: 5141
title: "Implement prefix increment in for update clauses"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: [080]
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement the narrow parser/frontend support needed for prefix increment expressions in `for` update clauses without initializers.

Problem: `SystemModuleForStatementNoInitializer.ts` currently fails with `UnsupportedSyntax` because the parser rejects `++i` in `for (; i < limit; ++i)`.

## Problem

The reference case is categorized as `module-system-amd`, but the current compiler failure occurs before module-system behavior is evaluated. The parser reaches the `for` statement update expression and rejects prefix increment as an unsupported expression.

## Current failure

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts
```

Current diagnostic:

```text
UnsupportedSyntax / parser-or-frontend-unsupported
unsupported expression: Some(SpannedToken { kind: Increment, span: Span { start: 105, end: 107 } }) at 107..108
```

Source context:

```ts
let i = 0;
let limit = 10;

for (; i < limit; ++i) {
    break;
}

for (; ; ++i) {
    break;
}
```

## Desired final state

The parser accepts prefix increment update expressions in `for` statements without initializers. If later module-system or emit behavior still blocks the reference case, that later failure is reported with its own precise issue-linked diagnostic.

## Scope

In scope:

- [ ] Parse prefix `++identifier` as an expression where `for` update clauses are accepted.
- [ ] Preserve existing behavior for unsupported prefix increment contexts not covered by this slice.
- [ ] Add a focused parser or fixture test for `for (; i < limit; ++i)` and `for (; ; ++i)`.
- [ ] Confirm issue 080 remains closed as the generated parent bucket.

Out of scope:

- Full update-expression semantics.
- Module-system AMD emit.
- General `for` statement lowering beyond the current parser blocker.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`
- `issues/done/080-implement-SystemModuleForStatementNoInitializer.md`

Do not touch:

- `crates/backend-wasm/`
- module-system emit code

## Acceptance criteria

- [ ] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts` no longer reports `unsupported expression` for `Increment`.
- [ ] A focused parser or fixture test covers `for (; i < limit; ++i)` and `for (; ; ++i)`.
- [ ] Unsupported prefix/update expression cases outside the accepted shape still produce precise diagnostics.
- [ ] Issue 080 remains closed as a superseded generated bucket.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend parser
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/SystemModuleForStatementNoInitializer.ts --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none unless the reference case advances to a separate module-system blocker
