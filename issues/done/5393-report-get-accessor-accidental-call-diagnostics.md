---
id: 5393
title: "Report get accessor accidental call diagnostics"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TypeScript-compatible TS6234 diagnostic when a typed class receiver
calls a `get` accessor as though it were callable.

## Problem

Current diagnostic: `UnsupportedSyntax: issue-211: unknown receiver class for
method property` at `x.property()` in
`accessorAccidentalCallDiagnostic.ts`.

Fresh triage on 2026-05-08 shows tokens and AST succeed. Lowering then reaches
the generic unknown-receiver fallback. TypeScript instead reports TS6234:
calling a `get` accessor is invalid and the user should use it without `()`.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts
```

Equivalent repo task:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts
```

Representative source:

```ts
class Test24554 {
    get property(): number { return 1; }
}
function test24554(x: Test24554) {
    return x.property();
}
```

Observed evidence:

```text
tokens: ok
ast: ok; ClassDecl Test24554 has "get property"; Return contains Call(Member(x, property))
lowering: UnsupportedSyntax issue-211 unknown receiver class for method property
oracle: TS6234 at property in x.property()
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=class-accessor:1
```

## Desired final state

The pipeline classifies this getter call before the generic unknown-receiver
fallback and emits a source-spanned accidental-call diagnostic.

## Scope

In scope:

- [ ] Preserve getter metadata for `get property(): number`.
- [ ] Classify `x.property()` against receiver type `Test24554` before `issue-211`.
- [ ] Emit a TS6234-like diagnostic while keeping ordinary class member calls unchanged.

Out of scope:

- Callable auto-accessor fields, tracked by issue 5322.
- Missing instance/static member diagnostics, tracked by issue 5261.
- Full TypeScript call-signature checking.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/ir/src/lowered/`
- `crates/frontend/src/`
- `crates/cli/tests/`

Do not touch:

- backend/runtime code unless this slice produces a supported lowered call shape

## Acceptance criteria

- [ ] `accessorAccidentalCallDiagnostic.ts` no longer reports `issue-211` for `property`.
- [ ] A focused fixture covers `class C { get property(): number { return 1; } } function f(x: C) { x.property(); }`.
- [ ] The diagnostic is source-spanned at `property` or `x.property()`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(accessor) or test(call) or test(diagnostic)'
cargo nextest run -p ts2wasm-cli -E 'test(accessor) or test(call) or test(diagnostic)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/accessorAccidentalCallDiagnostic.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Notes

Split from generated bucket `565` on 2026-05-08. Related broad bucket `422`
does not own this exact TS6234 diagnostic slice.
