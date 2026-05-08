---
id: 5207
title: "Support ambient interface filter receivers"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

After ambient value declarations are modeled for name resolution, `booleanFilterAnyArray.ts` advances to a method receiver classification gap. The call `anys.filter(Bullean)` is parsed and name-resolved, but lowering rejects it because `anys` has no concrete receiver class tracked by the current builtin resolver.

## Problem

Problem: `reference/typescript/tests/cases/compiler/booleanFilterAnyArray.ts` now reports `issue-211: unknown receiver class for method \`filter\`` for an ambient interface-typed receiver after `declare let anys: Ari<any>;` is resolved.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/booleanFilterAnyArray.ts
```

Current diagnostic:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `filter` at 388..408
```

Source context:

```ts
declare var Bullean: BulleanConstructor;
declare let anys: Ari<any>;
var xs: Ari<any>;
var xs = anys.filter(Bullean)
```

Fresh triage evidence after issue 5161:

- AST contains `AmbientValueDecl { name: "Bullean" }` and `AmbientValueDecl { name: "anys" }`.
- Name resolution no longer reports `UnresolvedName` for `anys` or `Bullean`.
- Lowering stops at receiver classification for `anys.filter(Bullean)`.

## Desired final state

The compiler has a deliberate supported behavior or issue-linked diagnostic for `.filter(...)` on ambient interface-like receivers such as `Ari<any>`, without regressing concrete array `.filter(...)` behavior.

## Scope

In scope:

- [x] Triage whether ambient interface receivers should lower through the existing array-filter path, a typed placeholder, or a narrower diagnostic.
- [x] Add focused coverage for `declare let anys: Ari<any>; var xs = anys.filter(Bullean);`.
- [x] Re-run `booleanFilterAnyArray.ts` triage and record the next blocker or pass state.

Out of scope:

- Full TypeScript structural type checking.
- General method-call support for all interface-typed receivers.
- Ambient value declaration name-resolution; issue 5161 owns that prerequisite.

## Affected paths

Expected:

- `crates/ir/src/lowered/`
- `crates/ir/src/builtin_resolver.rs`
- `crates/ir/src/builtin_resolver_outer.rs`
- `fixtures/`

Do not touch:

- `crates/frontend/src/parser/` unless fresh triage proves a parser gap.

## Acceptance criteria

- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/booleanFilterAnyArray.ts` no longer reports `UnresolvedName` for `anys`.
- [x] The same triage no longer reports the generic `issue-211: unknown receiver class for method \`filter\`` blocker for `anys.filter(Bullean)`, or it reports a narrower issue-linked diagnostic with a new owner.
- [x] Existing concrete array `.filter(...)` fixtures and tests continue to pass.
- [x] Docs/current-state/issues are synchronized when status or design changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/booleanFilterAnyArray.ts
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/booleanFilterAnyArray.ts --detail
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

Split while closing issue 5161 after fresh triage proved the ambient `declare let` name-resolution gap had advanced to receiver classification.

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


## False-done audit

Date: 2026-05-07

Classification: truly-done.

Audit result: retained in issues/open/. Implementation commits confirmed.
