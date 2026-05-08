---
id: 1279
title: "Implement Collisioncodegenmodulewithmembervariable"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1279.

## Summary

Triage collisionCodeGenModuleWithMemberVariable across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results previously showed 1 case failing in directory `collisionCodeGenModuleWithMemberVariable` with diagnostics: import-export. Fresh triage now reaches a narrower name-resolution blocker that is already covered by issue 5287.

Problem: collisionCodeGenModuleWithMemberVariable needs same-file namespace value binding for qualified access, already tracked by issue 5287.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberVariable.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberVariable.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by issue 5287. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing issue 5287
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed bucket

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Superseded by `issues/done/5287-bind-namespace-declarations-for-qualified-value-access.md`
- [x] Smart triage evidence below includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript evidence
- [x] Superseding issue 5287 acceptance names the same diagnostic family and qualified namespace access behavior

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberVariable.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberVariable.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/done/5287-bind-namespace-declarations-for-qualified-value-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberVariable.ts`

## Duplicate detection

- `issues/done/5287-bind-namespace-declarations-for-qualified-value-access.md` owns same-file non-ambient namespace value binding for qualified access.

## Smart triage

Reproduction:
`python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberVariable.ts`.

Focused coverage:
`python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberVariable.ts --detail --no-dashboard-data`.

Current diagnostic:

```text
UnresolvedName: unresolved name: `m1` at 91..93
```

Focused coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Representative source:

```ts
namespace m1 {
    export var m1 = 10;
    var b = m1;
}
var foo = m1.m1;
```

Compiler evidence:

```text
tokens: ok
ast: ok; retained AST contains `var foo = m1.m1`
resolved: fails in resolve_names with UnresolvedName for namespace value `m1`
visible symbols: exported binding m1, local b, top-level foo
```

TypeScript oracle evidence:

```text
typescript ok; diagnostics: []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- superseded by `issues/done/5287-bind-namespace-declarations-for-qualified-value-access.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionCodeGenModuleWithMemberVariable.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; current UnresolvedName blocker is covered by issue 5287
date: 2026-05-07
```

Remaining risks:

- Issue 5287 must cover exported namespace variables as well as its existing
  exported function/class examples, or split a child if implementation reveals
  a narrower exported-variable blocker.
