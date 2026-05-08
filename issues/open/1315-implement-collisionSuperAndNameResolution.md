---
id: 1315
title: "Implement Collisionsuperandnameresolution"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: []
blocks: [5339]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1315.

## Summary

Closed after splitting the current parser/name-resolution blocker into `issues/done/5339-preserve-var-after-object-type-declaration.md`.

## Problem

Reference test results show 1 case failing in directory `collisionSuperAndNameResolution` with diagnostics: name-resolution. Fresh triage shows the resolver reports `UnresolvedName` for `_super`, but the AST evidence shows the parser dropped the top-level `var _super = 10` after parsing the preceding typed `var console: { ... }` declaration.

Problem: `collisionSuperAndNameResolution.ts` currently reports `UnresolvedName: unresolved name: \`_super\`` because a type-only `var console: { ... }` declaration consumes the following initialized `var _super = 10` declaration instead of preserving it as a separate binding.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation should proceed through `issues/done/5339-preserve-var-after-object-type-declaration.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] Child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts
```

Not run:

- `cargo fmt --all --check` (not run; issue metadata only)
- `cargo nextest run` (not run; issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/done/5339-preserve-var-after-object-type-declaration.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts`

## Duplicate detection

- Broad name-resolution issues were found by feature label, but none owns this exact `collisionSuperAndNameResolution.ts` parser/name-resolution blocker.
- `issues/done/5339-preserve-var-after-object-type-declaration.md` owns this current blocker.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: collisionSuperAndNameResolution

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts`
```

Failure location:

```text
error: [UnresolvedName] unresolved name: `_super`
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
```

Source context:

```ts
var console: {
    log(message: any);
}
var _super = 10; // No error
class base {
}
class Foo extends base {
```

Compiler evidence:

```text
tokens: ok; includes `var console: { ... }`, following `var _super = 10`, class base, and class Foo extends base
ast: ok but wrong shape; top-level statements contain `Let console = Number(10)` and no separate `_super` binding
resolved: fails in resolve_names with UnresolvedName for `_super`
```

TypeScript oracle:

```text
diagnostics: TS2403 for duplicate `console` declaration only
binding `_super`: number
```

## Completion evidence


Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts
result: UnresolvedName for `_super`; split to issue 5339
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionSuperAndNameResolution.ts --detail --no-dashboard-data
result: unsupported=1 unsupported_diagcodes=UnresolvedName:1 unsupported_features=name-resolution:1
date: 2026-05-07
```

Remaining risks:

- TypeScript oracle still reports a duplicate `console` declaration diagnostic in the current toolchain; this split only tracks the compiler blocker that prevents `_super` lookup from reaching the expected later behavior.
