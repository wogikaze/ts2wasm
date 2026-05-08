---
id: 5483
title: "Support dependency named exports of default-import bindings"
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

Allow a dependency virtual file to validate `export { d }` when `d` is the
local binding from `import d, { a } from "./a.cjs"`.

Split from generated bucket `3578`.

## Problem

`nodeNextCjsNamespaceImportDefault1.ts` and
`nodeNextCjsNamespaceImportDefault2.ts` have this shape:

```ts
import d, {a} from './a.cjs';
import * as ns from './a.cjs';
export {d, a, ns};
```

Fresh triage stops before NodeNext interop semantics:

```text
UnsupportedSyntax: issue-5005: dependency module `export { d }` references unknown local binding `d` at 69..70
```

Problem: dependency-module named export validation does not recognize the
default-import local `d` as an exportable value binding.

## Current Failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts
```

The second path has the same first blocker. Coverage for both paths reports `UnsupportedModule/import-export`.

Compiler evidence:

```text
tokens/ast: ok through ImportDefaultNamed local d/a, ImportNamespace ns, ExportNamed d/a/ns
module build: issue-5005 dependency module `export { d }` references unknown local binding `d`
resolved dump: later issue-232 missing local module `./a.cjs`
```

## Desired Final State

The representative paths no longer report issue-5005 unknown local binding for
`export { d }`. Any later `a`, `ns`, `./a.cjs`, or NodeNext interop blocker is recorded separately.

## Scope

In scope:

- [ ] Register `ImportDefaultNamed` default locals as exportable value bindings in dependency-module named export validation.
- [ ] Preserve export metadata for `import d, { a } from "./dep"; export { d };`.
- [ ] Add one focused module graph regression for a dependency module re-exporting a default-import local.
- [ ] Re-triage representative paths and record the next diagnostic.

Out of scope:

- Named-import local `a` if it remains blocked after `d`.
- Namespace-import local `ns`, tracked by `issues/open/5469-support-named-exports-of-namespace-imports-in-dependency-modules.md`.
- Local virtual `./a.cjs` resolution and NodeNext interop.

## Acceptance criteria

- [ ] A dependency fixture for `import d from "./dep"; export { d };` no longer reports issue-5005 unknown local binding.
- [ ] Missing local export names still produce clear diagnostics.
- [ ] Both `nodeNextCjsNamespaceImportDefault*.ts` paths advance past `export { d }`.
- [ ] The next blocker is recorded here or split if outside scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(module) or test(import) or test(export)'
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts --detail --no-dashboard-data
```
