---
id: 3435
title: "Implement Namespaceswithtypealiasonlyexportsmerge"
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
now passes. The first remaining TypeScript oracle mismatch is split to
`issues/open/5443-report-duplicate-type-alias-identifiers.md`.

## Problem

Reference test results show 1 cases fail in directory `namespacesWithTypeAliasOnlyExportsMerge` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Fresh coverage shows `namespacesWithTypeAliasOnlyExportsMerge.ts` no longer
fails with `import-export`; it is a build pass. TypeScript still reports
TS2300 duplicate identifier diagnostics for type-only declarations, followed by
TS2451 redeclaration diagnostics for repeated `declare const try*` names.

Problem: the stale generated import/export blocker is gone, and the first
semantic parity gap belongs to a focused duplicate type-alias identifier issue.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts --detail
```

## Desired final state

This generated bucket is closed. Implement semantic parity from
`issues/open/5443-report-duplicate-type-alias-identifiers.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the stale import/export blocker is gone
- [x] Split the first remaining semantic oracle mismatch into a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/5443-report-duplicate-type-alias-identifiers.md`

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts
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

- [x] created: `issues/open/5443-report-duplicate-type-alias-identifiers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts`

## Duplicate detection

- `issues/open/5438-support-named-exports-of-local-interfaces.md` is related
  type-only named export work, but it covers currently failing unknown local
  bindings for interface exports. This reference now build-passes and exposes
  duplicate identifier diagnostics instead.
- No exact owner existed for the first TS2300 duplicate type-alias identifier
  diagnostic, so issue 5443 was created.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts --detail --no-dashboard-data

result:
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts: build_pass
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts

result:
UnsupportedSyntax: multi-section file has no module bodies
tokens are ok; retained AST/resolved dumps contain only empty export markers; TypeScript oracle reports TS2300 and TS2451 diagnostics.
```

Source context:

```ts
// @filename: constAndNS.ts
type A = number;
declare const Q: number;
declare namespace Q {
    export { A };
}
declare const try1: Q.A;

// @filename: circularWithUses.ts
type A = string;
type B = number;
declare namespace NS1 {
    export { NS2, A };
}
declare namespace NS2 {
    export { NS1, B };
}
export {};
declare const try1: NS1.A;
```

Compiler evidence:

```text
tokens: ok through type aliases, declare consts, declare namespaces, type-only namespace exports, and final declare const uses
ast: retained statements are only empty ExportNamed markers for `export {}`
resolved: retained statements lower to Undefined expressions
coverage: build_pass=1, unsupported=0
```

TypeScript oracle evidence:

```text
TS2300: Duplicate identifier 'A'.       // first `type A = number`
TS2451: Cannot redeclare block-scoped variable 'try1'.
TS2451: Cannot redeclare block-scoped variable 'try2'.
TS2451: Cannot redeclare block-scoped variable 'try3'.
TS2451: Cannot redeclare block-scoped variable 'try4'.
TS2300: Duplicate identifier 'NS2'.
TS2300: Duplicate identifier 'NS1'.
TS2300: Duplicate identifier 'A'.       // second `type A = string`
TS2300: Duplicate identifier 'NS2'.
TS2300: Duplicate identifier 'NS1'.
```

## Completion evidence

Closed as stale import/export bucket; the first current semantic mismatch was
split to issue 5443.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts
result: pass; compiler build-passes in coverage, TypeScript oracle reports TS2300/TS2451 split first to issue 5443
date: 2026-05-08
```

Remaining risks:

- After issue 5443 advances this path, later TS2451 block-scoped redeclarations
  for `try1` through `try4` and duplicate namespace identifiers for `NS1` /
  `NS2` may need narrower follow-up issues.
