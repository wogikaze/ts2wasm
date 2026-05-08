---
id: 5172
title: "Report unresolved implements in erased namespace"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Report the TypeScript-style unresolved name diagnostic for the tiny `bind1.ts` namespace/class reference.

## Problem

`bind1.ts` now builds successfully, but TypeScript reports `TS2304: Cannot find name 'I'.` for `export class C implements I {}`. The parser currently erases the namespace body wholesale, so the frontend never records or diagnoses the unresolved `implements` type name.

Problem: erased namespace declarations can hide unresolved class `implements` clauses and produce a false build pass.

## Current failure

Representative reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bind1.ts
```

Current compiler result:

```text
BuildPass: ts2wasm build succeeded
```

Source context:

```ts
namespace M {
    export class C implements I {} // this should be an unresolved symbol I error
}
```

Compiler evidence:

- Token dump recognizes `namespace`, `export`, `class`, `implements`, and `I`.
- AST and resolved dumps are empty because the namespace body is erased.
- TypeScript oracle reports `TS2304: Cannot find name 'I'.` at the `I` token.

## Desired final state

The frontend preserves enough evidence from erased namespace bodies to report the unresolved `implements I` diagnostic for this reference case instead of returning a build pass.

## Scope

In scope:

- [x] Detect `class ... implements <Ident>` inside erased namespace declarations.
- [x] Report a source-spanned unresolved-name diagnostic for missing `implements` type names.
- [x] Add focused coverage for `namespace M { export class C implements I {} }`.

Out of scope:

- Full namespace runtime or emit support.
- Type checking complete `implements` conformance when the interface exists.
- Broad import/export module syntax work owned by issue `432`.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- semantic/reference triage diagnostics if a new diagnostic mapping is needed

Do not touch:

- ES module import/export loading
- Function `bind`, `call`, or `apply` runtime behavior

## Acceptance criteria

- [x] A focused frontend test covers `namespace M { export class C implements I {} }`.
- [x] The diagnostic is source-spanned at `I` and no longer produces an empty AST/resolved build pass for the representative case.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bind1.ts` no longer reports `BuildPass` when TypeScript reports `TS2304`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend implements
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bind1.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from generated bucket `1057` on 2026-05-06. The bucket title is `Bind`, but current evidence is about TypeScript bind/checker terminology, not `Function.prototype.bind`.

## Completion evidence

Commits:

- `HEAD (final issue commit)`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-frontend implements
result: 2 passed, 0 failed
date: 2026-05-06

command: cargo build -q -p ts2wasm-cli
result: pass (warning: compile_source_with_emit is dead code)
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bind1.ts
result: UnresolvedName at 65..66; no longer BuildPass
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

