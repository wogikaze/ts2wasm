---
id: 5469
title: "Support named exports of namespace imports in dependency modules"
type: feature
area: compiler/module-graph
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Allow dependency modules to validate and lower `export { B }` when `B` is a
local namespace-import value binding from `import * as B from "./b"`.

Split from generated bucket `3523`.

## Problem

`noCrashOnImportShadowing.ts` has virtual files where `a.ts` imports a sibling
module as a namespace and then re-exports that namespace binding:

```ts
// @filename: b.ts
export const zzz = 123;

// @filename: a.ts
import * as B from "./b";
interface B {
    x: string;
}
export { B };
```

Fresh triage reaches module export validation for the dependency `a.ts` section
and reports that the export list references an unknown local binding:

```text
UnsupportedSyntax: issue-5005: dependency module `export { B }` references unknown local binding `B` at 100..101
```

Problem: dependency-module named export validation does not recognize namespace
import bindings as local exportable values, so the compiler stops at an
issue-5005 unknown-local-binding boundary before import shadowing diagnostics or
later multi-file behavior can be triaged.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnImportShadowing.ts
```

Observed triage:

```text
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Feature label: import-export
Failure: issue-5005 dependency module `export { B }` references unknown local binding `B`
```

Focused coverage currently reports the same reference as unsupported under the
broader duplicate-local bucket:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
```

Compiler evidence:

```text
tokens: ok through export const zzz, import namespace B, interface B, export { B }, import { B }, duplicate const x, and namespace import OriginalB
ast: contains ImportNamespace local B from "./b" and ExportNamed local/exported B
resolved/module build: stops at issue-5005 for dependency module export { B }
```

TypeScript oracle evidence:

```text
TS2300 duplicate identifier B at the namespace import
TS2300 duplicate identifier B at the interface
TS2451 duplicate block-scoped variable x at both const x declarations
TS2307 cannot find local virtual modules "./a" / "./b" in the oracle run
```

## Desired Final State

A dependency module can export a local namespace-import value binding through a
named export list. The representative reference no longer reports issue-5005
unknown local binding for `export { B }`; it either advances to a TypeScript-like
duplicate identifier diagnostic or to the next already tracked multi-file
module-resolution blocker.

## Scope

In scope:

- [ ] Register `ImportNamespace` locals as exportable value bindings for dependency-module named export validation.
- [ ] Preserve export metadata for `import * as B from "./b"; export { B };` in a dependency module.
- [ ] Add a focused module graph regression with a dependency module that re-exports a namespace import.
- [ ] Re-triage `noCrashOnImportShadowing.ts` and record the next diagnostic after this issue-5005 boundary.

Out of scope:

- Named exports of local type-only interface declarations, tracked by `issues/open/5438-support-named-exports-of-local-interfaces.md`.
- Local imports between `// @filename:` virtual sections, tracked by `issues/open/5229a-resolve-imports-between-filename-sections.md`.
- TS2451-style duplicate `const` diagnostics, tracked by `issues/open/5412a-report-ts2451-duplicate-const-filename-sections.md`.
- Full TypeScript namespace/type/value merge semantics outside this exact namespace-import export boundary.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- `crates/ir/src/name_resolver.rs`
- focused compiler/module tests

Do not touch:

- backend/runtime lowering unless focused module tests prove the export metadata already reaches backend emission
- package or on-disk module resolution

## Acceptance criteria

- [ ] A focused fixture for `import * as B from "./b"; export { B };` in a dependency module no longer reports issue-5005 unknown local binding.
- [ ] The exported namespace binding remains a runtime value and does not rely on type-only interface declarations.
- [ ] Unsupported or missing local export names still produce clear diagnostics.
- [ ] `noCrashOnImportShadowing.ts` is re-triaged and the next diagnostic is recorded in this issue or split if outside scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(module) or test(namespace)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnImportShadowing.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnImportShadowing.ts --detail --no-dashboard-data
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

- [x] existing issues 5438, 5229, and 5412 cover known later boundaries

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
