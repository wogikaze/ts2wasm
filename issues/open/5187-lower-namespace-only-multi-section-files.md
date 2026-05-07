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

- [ ] Preserve a non-empty lowered module body for a `// @Filename:` section containing namespace declarations.
- [ ] Keep declaration-only `.d.ts` sections available for reference lookup or emit a focused unsupported diagnostic with section name evidence.
- [ ] Add focused coverage for a two-section `namespace` plus `declare namespace` reference fixture.

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

- [ ] A focused test covers a multi-section file with `namespace C` and `declare namespace A`.
- [ ] The representative fixture no longer reports `multi-section file has no module bodies`.
- [ ] The next diagnostic includes the relevant section name or source span instead of an unspanned empty-body error.

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

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from generated bucket `1076` on 2026-05-06. Broader namespace semantics remain separate work; this issue only removes the empty multi-section body blocker.
Also owns the direct-triage empty-body guard for `issues/done/3344-implement-moduleKeywordDeprecated.md`: fresh coverage for `moduleKeywordDeprecated.ts` is `build_pass`, but direct triage still reports `multi-section file has no module bodies` before module-keyword deprecation diagnostics can be inspected.

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
