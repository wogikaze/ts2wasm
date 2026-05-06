---
id: 5187
title: "Lower namespace-only multi-section files"
type: feature
area: compiler
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`blockScopedNamespaceDifferentFile.ts` is split by `// @Filename:` into `test.ts` and `typings.d.ts`, but the compiler drops both sections as empty module bodies and reports `multi-section file has no module bodies`.

## Problem

The representative file has two virtual sections:

```ts
// @Filename: test.ts
namespace C {
    export class Name {
        static funcData = A.AA.func();
        static someConst = A.AA.foo;
    }
}

// @Filename: typings.d.ts
declare namespace A {
    namespace AA {
        function func(): number;
        const foo = "";
    }
}
```

`split_file_name_sections` finds the sections, but `lower_source_as_module_body` only keeps rewritten static-module bodies. Namespace-only and declaration-only sections are discarded, leaving `build_multi_section_file` with no modules.

Problem: multi-section TypeScript reference files that contain namespace declarations but no static imports/exports are reduced to an empty program before their namespace diagnostics can be triaged.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedNamespaceDifferentFile.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: multi-section file has no module bodies
```

Compiler evidence:

- Tokens include `namespace C`, `export class Name`, and `declare namespace A`.
- AST dump returns an empty program.
- Resolved dump returns an empty program.
- The failure has no span because all virtual sections were discarded.

TypeScript oracle evidence:

```text
TS2729: Property 'AA' is used before its initialization.
```

The oracle reports two diagnostics at `A.AA` static member initializers in `test.ts`.

## Desired final state

The compiler keeps namespace-only multi-section bodies observable enough for the next namespace/scope diagnostic to surface. The representative case should no longer fail with `multi-section file has no module bodies`.

## Scope

In scope:

- Outcome: used the focused unsupported diagnostic path instead of preserving a non-empty lowered module body for namespace-only sections.
- [x] Keep declaration-only `.d.ts` sections available for reference lookup or emit a focused unsupported diagnostic with section name evidence.
- [x] Add focused coverage for a two-section `namespace` plus `declare namespace` reference fixture.

Out of scope:

- Full namespace ownership/runtime lowering.
- AMD/outFile emit.
- Implementing TS2729 property-initialization diagnostics.
- General module-resolution support for imported files.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/cli/tests/official_corpora.rs`
- `fixtures/`

Do not touch:

- Backend namespace emit unless the focused test proves the empty-body guard cannot be bypassed earlier.
- Static ES module import/export lowering.

## Acceptance criteria

- [x] A focused test covers a multi-section file with `namespace C` and `declare namespace A`.
- [x] The representative fixture no longer reports `multi-section file has no module bodies`.
- [x] The next diagnostic includes the relevant section name or source span instead of an unspanned empty-body error.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli --test official_corpora
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedNamespaceDifferentFile.ts
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

Split from generated bucket `1076` on 2026-05-06. Broader namespace semantics remain separate work; this issue only removes the empty multi-section body blocker.

## Completion evidence

Commits:

- `HEAD (final issue commit)`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli --test official_corpora
result: 3 passed, 0 failed, 1 skipped
date: 2026-05-06

command: cargo nextest run -p ts2wasm-compiler reports_namespace_only_multi_section_with_section_name
result: 1 passed, 0 failed
date: 2026-05-06

command: cargo build -q -p ts2wasm-cli
result: pass (warning: compile_source_with_emit is dead code)
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/blockScopedNamespaceDifferentFile.ts
result: UnsupportedRuntimeSubset with section `test.ts` and span 0..9; no longer `multi-section file has no module bodies`
date: 2026-05-06
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

