---
id: 5442
title: "Report mixed default function namespace merge diagnostic"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TS2395-style diagnostic when an ambient external module exports a
function/default binding and also declares a local namespace with the same
name.

## Problem

`namespaceNotMergedWithFunctionDefaultExport.ts` now build-passes in coverage,
but TypeScript reports TS2395 for the mixed exported/local declarations:

```ts
declare module 'replace-in-file' {
  export function replaceInFile(config: unknown): Promise<unknown[]>;
  export default replaceInFile;

  namespace replaceInFile {
    export function sync(config: unknown): unknown[];
  }
}
```

Current compiler evidence:

```text
coverage: build_pass=1
tokens: ok through exported function, default export, local namespace, and exported sync
ast/resolved: empty retained runtime AST for the declaration-only module body
```

TypeScript oracle evidence:

```text
TS2395: Individual declarations in merged declaration 'replaceInFile' must be all exported or all local.
```

Problem: ambient module declaration bodies are erased before declaration export
state is checked, so ts2wasm build-passes while TypeScript reports the mixed
exported/local merge diagnostic.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts
```

Current result:

```text
coverage build_pass=1
TypeScript oracle reports TS2395 at `export function replaceInFile` and at `namespace replaceInFile`
```

## Desired final state

The compiler preserves enough ambient module declaration export-state
information to reject the representative mixed merge instead of returning a
build pass.

## Scope

In scope:

- [ ] Track declaration names and exported/local state within an ambient
  `declare module "..." { ... }` body for the focused function plus namespace
  merge shape.
- [ ] Treat `export function replaceInFile(...)` and
  `export default replaceInFile` as exported declarations/references.
- [ ] Treat `namespace replaceInFile { ... }` without `export` as a local
  declaration of the same name.
- [ ] Report a source-spanned TS2395-style diagnostic at the conflicting
  `replaceInFile` declarations.
- [ ] Add focused coverage for the representative ambient module body.

Out of scope:

- Full TypeScript declaration merging.
- Runtime lowering for ambient external modules.
- General default export module emit.
- Same-namespace exported/local `var` merge diagnostics, tracked by
  `issues/open/5436-report-mixed-exported-local-namespace-vars.md`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- focused semantic/parser regression tests

Do not touch:

- backend/runtime ABI
- broad package/module resolution

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts` no longer silently build-passes in coverage while TypeScript reports TS2395.
- [ ] A focused fixture covers `declare module "m" { export function f(): void; export default f; namespace f {} }`.
- [ ] The diagnostic message or code identifies mixed exported/local declarations for `f`.
- [ ] Existing declaration-only ambient module fixtures that do not mix export state still build or erase as before.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend namespace
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts --detail --no-dashboard-data
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

Split from `issues/done/3432-implement-namespaceNotMergedWithFunctionDefaultExport.md`
on 2026-05-08 after fresh coverage showed the generated import/export blocker
was stale and the current mismatch is a false build-pass.

Related but distinct:

- `issues/open/5436-report-mixed-exported-local-namespace-vars.md` covers TS2395
  for same-namespace `var` declarations, not exported/default function plus
  local namespace merging inside an ambient external module.

## Completion evidence

Fill when implemented.
