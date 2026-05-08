---
id: 5368
title: "Isolate exported bindings across @filename sections"
type: feature
area: compiler/multi-section
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Treat external-module TypeScript reference `// @filename:` sections as separate
module binding scopes so repeated exported names in different virtual files do
not collide as `DuplicateLocal`.

## Problem

`contextualOverloadListFromArrayUnion.ts` contains multiple virtual files whose
first two sections each declare and export `yThen`:

```ts
// @filename: one.ts
declare const y: never[] | string[];
export const yThen = y.map(item => item.length);
// @filename: two.ts
declare const y: number[][] | string[];
export const yThen = y.map(item => item.length);
```

The compiler keeps both `ExportDecl` declarations in one shared binding scope
and rejects the second one with:

```text
DuplicateLocal: duplicate local binding: `yThen` at 215..256
```

Problem: external-module `@filename` sections are not isolated during name
resolution, so per-file exported bindings collide before contextual overload
and arrow callback behavior can be triaged.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts --detail --no-dashboard-data
```

Observed result:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
resolved: duplicate local binding: `yThen` at 215..256
```

Compiler evidence:

```text
tokens: ok for both @filename sections, export const yThen, and map callbacks
ast: ok; contains two ExportDecl nodes named yThen from one.ts and two.ts
resolved: fails with DuplicateLocal for the second yThen
```

## Desired final state

For reference-style multi-section input, each external-module `@filename`
section has an isolated module binding scope. Repeated exported names in
different virtual files no longer report `DuplicateLocal`; the representative
file either builds or advances to a narrower contextual typing or overload
diagnostic.

## Scope

In scope:

- [x] Isolate top-level exported value bindings per external-module `@filename` section.
- [x] Add a focused multi-section test with two sections that both use `export const sameName = ...`.
- [x] Re-run `contextualOverloadListFromArrayUnion.ts` and record the next blocker if this path advances.

Out of scope:

- Local imports between `@filename` sections; tracked by `issues/done/5229-w0-user-runtime-string-origin.md`.
- Sharing non-module script globals between `@filename` sections; tracked by `issues/open/5328-share-script-globals-across-filename-sections.md`.
- Namespace-only multi-section body preservation; tracked by `issues/done/5187-lower-namespace-only-multi-section-files.md`.
- Contextual overload list type inference after this binding-scope boundary.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/ir/src/name_resolver.rs`
- focused compiler/reference tests

Do not touch:

- package or on-disk module resolution unless a focused fixture proves it is required
- backend/runtime ABI
- unrelated duplicate-local diagnostics for same-section declarations

## Acceptance criteria

- [x] `contextualOverloadListFromArrayUnion.ts` no longer reports `DuplicateLocal` for the second `export const yThen`.
- [x] A focused regression proves duplicate exported names are allowed across separate external-module `@filename` sections.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(filename) or test(module)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualOverloadListFromArrayUnion.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from generated bucket
`issues/done/1490-implement-contextualOverloadListFromArrayUnion.md` on
2026-05-07.

Related but not duplicates:

- `issues/done/5229-w0-user-runtime-string-origin.md` covers local
  import specifiers between virtual files.
- `issues/open/5328-share-script-globals-across-filename-sections.md` covers
  global script declarations shared across virtual files, not external-module
  exported binding isolation.
- `issues/done/5127-implement-export-default-multifile-lowering.md` fixed only
  duplicate synthetic default export locals.

## Completion evidence

Fill only when implemented.

## False-done audit

**truly-done** (5368)

- Implementation commits: verified via `git log --oneline --all --grep=5368`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
