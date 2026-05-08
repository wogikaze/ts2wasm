---
id: 5433
title: "Report duplicate static class member modifiers"
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

Report a TypeScript-style diagnostic for duplicate `static` class member modifiers instead of falling through to the generic method parser error.

## Problem

`static static` class members currently report `UnsupportedSyntax: expected LeftParen` at the following member name instead of diagnosing the repeated `static` modifier.

Problem: duplicate `static` class member modifiers are parsed as a method/field boundary error instead of a source-spanned duplicate-modifier diagnostic.

## Current failure

Field representative:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/staticModifierAlreadySeen.ts
```

Current result:

```text
UnsupportedSyntax expected LeftParen, got Some(Ident("foo")) at static static foo = 1;
```

Error-bucket representative:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleClassPropertyModifiersErrors.ts
```

Current result:

```text
UnsupportedSyntax expected LeftParen, got Some(Ident("p3")) at static static p3;
```

## Source Context

```ts
class C {
    static static foo = 1;
    public static static bar() { }
}
```

```ts
class C {
    public public p1;
    private private p2;
    static static p3;
    public private p4;
    private public p5;
}
```

TypeScript oracle reports TS1434 on the second `static` in the representative cases. The compiler tokenizes both `static` modifiers but then expects a method parameter list at the member name.

## Desired final state

The frontend detects duplicate `static` class member modifiers and emits a source-spanned diagnostic at the second `static`. The representative files should no longer report `expected LeftParen` at the member name.

## Scope

In scope:

- [ ] Detect `static static` in class member modifier lists before method/field dispatch, with the diagnostic span on the repeated `static` keyword.
- [ ] Cover duplicate static on a field and on a method member.

Out of scope:

- Accessibility duplicate/conflict diagnostics such as `public public` or `public private`.
- Parsing ordinary modified static fields; tracked by #5271.
- Typed modified static fields; tracked by #5288.
- Runtime/static field lowering.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- focused diagnostic tests if parser diagnostics are exposed through CLI

Do not touch:

- runtime ABI
- backend emit
- static ES module resolution

## Acceptance criteria

- [ ] `staticModifierAlreadySeen.ts` and `multipleClassPropertyModifiersErrors.ts` no longer report `expected LeftParen`; both report a source-spanned duplicate/invalid `static` diagnostic.
- [ ] A focused parser test covers both `static static foo = 1;` and `public static static bar() {}`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/staticModifierAlreadySeen.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleClassPropertyModifiersErrors.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/staticModifierAlreadySeen.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleClassPropertyModifiersErrors.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Notes

Split from #3410 and #4252 on 2026-05-08. #5271 owns valid modified static field parsing; this issue owns only the repeated `static` diagnostic boundary.
