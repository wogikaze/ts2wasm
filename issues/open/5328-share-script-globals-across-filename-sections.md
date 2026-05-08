---
id: 5328
title: "Share script globals across @Filename sections for class namespace merge"
type: feature
area: compiler/name-resolution
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Share global-script declarations across TypeScript reference `// @Filename:`
sections so a class declared in one virtual script file can be resolved from a
later virtual script file that augments it with a namespace and reads a
qualified member.

This is the current blocker from `cloduleSplitAcrossFiles.ts`.

## Problem

`cloduleSplitAcrossFiles.ts` contains two virtual files:

```ts
// @Filename: cloduleSplitAcrossFiles_class.ts
class D { }

// @Filename: cloduleSplitAcrossFiles_module.ts
namespace D {
    export var y = "hi";
}
D.y;
```

The compiler recognizes the virtual sections, but lowers each section as a
separate module body with its own scope. The first section's `class D` is not
available while resolving `D.y` in the second section, and the non-ambient
namespace body is erased before it can supply a namespace value.

Problem: cross-section global script declarations are not shared, so the second
`@Filename` section reports `UnresolvedName` for `D` before class/namespace
merge behavior can be triaged.

## Current failure

Reproduction: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts`.

Focused coverage: `python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts --detail --no-dashboard-data`.

Observed result:

```text
error: [UnresolvedName] unresolved name: `D`
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Compiler evidence:

```text
tokens: ok for class D, namespace D, export var y, and D.y
ast: contains top-level ClassDecl D and Expr Member(Ident("D").y); the namespace
     declaration is currently erased
resolved: UnresolvedName for D during resolve_names
visible symbols before failure: []
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
diagnostics: []
topLevel: ClassDeclaration, ModuleDeclaration, ExpressionStatement
hint: y has type string
```

## Desired final state

For reference-style multi-section input, non-module script declarations from
one `@Filename` section are available to later script sections using the same
global program. The representative file should no longer stop at
`UnresolvedName: D`; it should either build successfully or advance to a
narrower source-spanned namespace/class-merge diagnostic.

## Scope

In scope:

- [ ] Preserve or share script-level value declarations such as `class D {}` across `@Filename` sections that are not external modules.
- [ ] Keep enough section metadata to explain cross-section lookup in diagnostics.
- [ ] Add focused coverage for `class D` in one virtual file and `D.y` in a later virtual file.
- [ ] Re-run `cloduleSplitAcrossFiles.ts` and record the next blocker if this path advances.

Out of scope:

- Local imports between `@Filename` sections; tracked by `issues/open/5229a-resolve-imports-between-filename-sections.md`.
- Namespace-only multi-section body preservation; tracked by `issues/open/5187-lower-namespace-only-multi-section-files.md`.
- Same-file namespace value binding; tracked by `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.
- Full declaration emit or AMD/outFile behavior.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/frontend/src/`
- `crates/ir/src/`
- focused compiler or reference coverage tests

Do not touch:

- static ES module/package resolution unless a focused fixture proves module sections need it
- backend namespace emit unless the resolver advances to a reviewed runtime lowering shape

## Acceptance criteria

- [ ] `cloduleSplitAcrossFiles.ts` no longer reports `UnresolvedName: unresolved name: D`.
- [ ] A focused test covers a reference-style file with `// @Filename: a.ts`, `class D {}`, `// @Filename: b.ts`, and `D.y`.
- [ ] The implementation distinguishes global script sections from external module sections when sharing bindings.
- [ ] Any next blocker from class/namespace merging or namespace member lowering is recorded here or split to a follow-up if outside this scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli --test official_corpora
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleSplitAcrossFiles.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/open/1254-implement-cloduleSplitAcrossFiles.md` on
2026-05-07.

Related but not duplicates:

- `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`
  covers same-file namespace declarations such as `namespace m1 { ... }`
  followed by `m1.fooExport()`.
- `issues/open/5187-lower-namespace-only-multi-section-files.md` covers
  namespace-only or declaration-only virtual sections that are currently dropped.
- `issues/open/5229a-resolve-imports-between-filename-sections.md` covers local
  import specifiers between virtual files.

## Completion evidence

Fill only when implemented.
