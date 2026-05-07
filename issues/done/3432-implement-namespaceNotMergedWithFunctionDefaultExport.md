---
id: 3432
title: "Implement Namespacenotmergedwithfunctiondefaultexport"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed this generated `import-export` bucket because the current compiler build
now passes. The remaining TypeScript oracle mismatch is split to
`issues/open/5442-report-mixed-default-function-namespace-merge-diagnostic.md`.

## Problem

Reference test results show 1 cases fail in directory `namespaceNotMergedWithFunctionDefaultExport` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Fresh coverage shows `namespaceNotMergedWithFunctionDefaultExport.ts` no longer
fails with `import-export`; it is a build pass. TypeScript still reports TS2395
because an exported function/default export is merged with a local namespace
declaration of the same name inside an ambient external module.

Problem: the stale generated import/export blocker is gone, and the remaining
semantic parity gap belongs to a focused mixed exported/local merge diagnostic
issue.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts --detail
```

## Desired final state

This generated bucket is closed. Implement semantic parity from
`issues/open/5442-report-mixed-default-function-namespace-merge-diagnostic.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the stale import/export blocker is gone
- [x] Split the remaining semantic oracle mismatch into a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/5442-report-mixed-default-function-namespace-merge-diagnostic.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5442-report-mixed-default-function-namespace-merge-diagnostic.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts`

## Duplicate detection

- `issues/open/5436-report-mixed-exported-local-namespace-vars.md` is related
  TS2395 work, but it covers same-namespace `var` declarations, not an ambient
  module exported function/default export merged with a local namespace.
- No exact owner existed for the `replaceInFile` TS2395 diagnostic, so issue
  5442 was created.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts --detail --no-dashboard-data

result:
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts: build_pass
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts

result:
UnsupportedSyntax: multi-section file has no module bodies
tokens are ok; retained AST/resolved dumps are empty; TypeScript oracle reports TS2395.
```

Source context:

```ts
declare module 'replace-in-file' {
  export function replaceInFile(config: unknown): Promise<unknown[]>;
  export default replaceInFile;

  namespace replaceInFile {
    export function sync(config: unknown): unknown[];
  }
}
```

Compiler evidence:

```text
tokens: ok through ambient external module, exported function replaceInFile, default export of replaceInFile, local namespace replaceInFile, and exported sync
ast/resolved: empty retained runtime AST for the declaration-only module body
coverage: executed=1, build_pass=1, unsupported=0
```

TypeScript oracle evidence:

```text
TS2395 at exported function replaceInFile:
Individual declarations in merged declaration 'replaceInFile' must be all exported or all local.

TS2395 at local namespace replaceInFile:
Individual declarations in merged declaration 'replaceInFile' must be all exported or all local.
```

## Completion evidence

Closed as stale import/export bucket; the current semantic mismatch was split to
issue 5442.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceNotMergedWithFunctionDefaultExport.ts
result: pass; compiler build-passes in coverage, TypeScript oracle reports TS2395 split to issue 5442
date: 2026-05-08
```

Remaining risks:

- General declaration merging diagnostics beyond this focused exported
  function/default export plus local namespace shape remain out of scope.
