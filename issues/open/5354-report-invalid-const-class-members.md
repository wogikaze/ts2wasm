---
id: 5354
title: "Report invalid const class members"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

`constInClassExpression.ts` currently builds, but TypeScript rejects the class body
member `const a = 4;` with TS1248. The current compiler accepts the file and
drops the invalid class member from the AST/resolved class body.

## Problem

Fresh triage for
`reference/typescript/tests/cases/compiler/constInClassExpression.ts` reports
`BuildPass`, while the TypeScript oracle reports:

```text
TS1248: A class member cannot have the 'const' keyword.
```

Problem: invalid `const` class members in class expressions are silently
accepted instead of producing a TypeScript-compatible diagnostic.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constInClassExpression.ts
```

Current source:

```ts
// @target: es2015
let C = class {
    const a = 4;
};
```

Current compiler behavior:

```text
Diagnostic: BuildPass / pass
AST: ClassDecl { name: "C", body: [], ... }
Resolved: ClassDecl { name: "C", methods: [], statics: [], ... }
```

TypeScript oracle:

```text
TS1248 at line 3, character 11: A class member cannot have the 'const' keyword.
```

## Desired Final State

The compiler reports an explicit diagnostic for `const` used as a class member
modifier/declaration in class expression bodies, instead of accepting the file
as a build pass or silently dropping the member.

## Scope

In scope:

- [ ] Add parser diagnostic handling for `const` in class expression bodies where a class member is being
      parsed.
- [ ] Report an explicit diagnostic aligned with TS1248 wording or an internal
      equivalent that reference triage can classify.
- [ ] Preserve normal class expression AST representation for valid members.
- [ ] Add focused frontend/compiler regression coverage for `let C = class {
      const a = 4; };`.
- [ ] Re-run the reference triage and confirm the result is no longer
      `BuildPass`.

Out of scope:

- Full TypeScript diagnostic-code parity.
- Static `static const H = 1;` class declaration coverage already represented
  by prior done buckets such as
  `issues/done/547-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md`.
- General class field implementation beyond the invalid `const` modifier case.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/compiler/src/lib.rs` only if diagnostic mapping belongs after parse
- `scripts/run/reference-triage.py` only if classification needs a new mapping

Do not touch:

- backend/runtime class lowering unless focused tests prove the diagnostic must
  be emitted after lowering

## Acceptance criteria

- [ ] `reference/typescript/tests/cases/compiler/constInClassExpression.ts`
      no longer reports `BuildPass` in `reference-triage`.
- [ ] A focused parser/frontend diagnostic test for `class { const a = 4; }` reports an
      invalid class-member `const` diagnostic.
- [ ] Valid class expressions with ordinary methods still parse.
- [ ] The invalid member is not silently dropped from the class body AST representation.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend class
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constInClassExpression.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constInClassExpression.ts --detail --no-dashboard-data
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

Split from generated bucket `1461` on 2026-05-07 after fresh triage showed the
file now builds but still lacks TypeScript semantic diagnostic parity.
