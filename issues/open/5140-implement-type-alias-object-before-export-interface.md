---
id: 5140
title: "Implement type alias object parsing before exported interface"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: [074]
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Implement the narrow parser support needed for `DeclarationErrorsNoEmitOnError.ts`: a semicolonless TypeScript object-type alias followed by an exported interface declaration.

Problem: `DeclarationErrorsNoEmitOnError.ts` currently fails with `UnsupportedSyntax` because the parser treats `type T = { x : number }` as an unterminated type alias when `export interface I` follows.

## Problem

The TypeScript parser path does not accept an object type literal body in a type alias when the alias is not terminated with an explicit semicolon. The failing reference case is small and isolated, so it should be implemented as a focused parser slice instead of staying hidden inside issue 074.

## Current failure

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts
```

Current diagnostic:

```text
UnsupportedSyntax / parser-or-frontend-unsupported
unterminated TypeScript type alias declaration at 92..96
```

Source context:

```ts
type T = { x : number }
export interface I {
    f: T;
}
```

## Desired final state

The parser accepts the `DeclarationErrorsNoEmitOnError.ts` type alias/interface sequence far enough to remove the `unterminated TypeScript type alias declaration` diagnostic. Any later unsupported construct must have its own precise issue-linked diagnostic or child issue.

## Scope

In scope:

- [ ] Parse object type literal members inside a TypeScript type alias.
- [ ] Accept newline/ASI termination for a type alias before `export interface`.
- [ ] Add a focused parser or reference fixture covering the exact source shape.
- [ ] Preserve existing TypeScript type-alias diagnostics for genuinely malformed aliases.

Out of scope:

- Full TypeScript type checker support.
- Emitting declaration files.
- General interface implementation beyond parsing enough to avoid the current alias diagnostic.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/`
- `issues/done/074-implement-DeclarationErrorsNoEmitOnError.md`

Do not touch:

- `crates/backend-wasm/`
- unrelated runtime builtins

## Acceptance criteria

- [ ] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts` no longer reports `unterminated TypeScript type alias declaration`.
- [ ] A focused test or fixture covers `type T = { x : number }` followed by `export interface I { f: T; }`.
- [ ] Malformed type aliases still reject with a precise parser diagnostic.
- [ ] Issue 074 remains closed as a superseded generated bucket.

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
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/DeclarationErrorsNoEmitOnError.ts --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none unless triage exposes a later independent declaration emit or interface parser blocker
