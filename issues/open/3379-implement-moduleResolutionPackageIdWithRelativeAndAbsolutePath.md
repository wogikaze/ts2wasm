---
id: 3379
title: "Implement Moduleresolutionpackageidwithrelativeandabsolutepath (audit reopened #3379)"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432, 5402]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage moduleResolutionPackageIdWithRelativeAndAbsolutePath across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case in
`moduleResolutionPackageIdWithRelativeAndAbsolutePath`. Fresh triage confirms
the current first blocker is the first virtual
`/shared/node_modules/troublesome-lib/package.json` section being parsed as
TypeScript/JavaScript source and failing at the JSON property colon.

Problem: this generated bucket is superseded by existing package-json section
owner issue `5402`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts --detail
```

## Desired final state

This generated bucket is closed. The actionable first blocker is tracked by
issue `5402`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede with existing implementation-ready issue `5402`
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the superseding issue note

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
- [x] Superseding issue `5402` contains exact `reference-triage` ownership evidence
- [x] Superseding issue includes failing path, diagnostic code, source context, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; this close only moves a
  generated triage bucket and updates issue metadata, with no Rust source
  changes.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by existing issue `5402`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated manually on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-coverage tsc \
  --path-filter reference/typescript/tests/cases/compiler/moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts \
  --detail --no-dashboard-data
```

Result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-resolution:1
per-file: moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts => UnsupportedSyntax / module-resolution
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts
```

Observed:

```text
diagnosis: UnsupportedSyntax / parser-or-frontend-unsupported
message: expected Semicolon, got Some(Colon) at 12..13
actual AST/resolved error: expected Semicolon, got Some(Colon) at 203..204
source:
// @filename: /shared/node_modules/troublesome-lib/package.json
{
    "name": "troublesome-lib",
    "version": "1.17.1"
}
```

Compiler evidence:

```text
tokens: ok for the package.json body and later imports/classes
ast/resolved: fail on the first package.json property colon
TypeScript oracle reports TS1005/TS2695 for package.json bodies and later
module-resolution / duplicate identifier diagnostics, none of which are
reachable before skipping package.json sections
```

Existing issue `5402` exactly covers virtual `package.json` sections being
parsed as code before later package, path-mapping, or module-resolution
diagnostics become reachable.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts --detail --no-dashboard-data
result: pass; reproduced UnsupportedSyntax/module-resolution first blocker
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionPackageIdWithRelativeAndAbsolutePath.ts
result: pass; first actionable blocker is virtual package.json parsing, superseded by issue 5402
date: 2026-05-08
```

Remaining risks:

- After issue `5402`, this reference may advance to path mapping,
  package-id identity, duplicate declarations, or virtual node_modules
  resolution blockers.

## Close note

Superseded by issue `5402`. Fresh triage stops in the first virtual
`package.json` body before package-id relative/absolute path behavior is
actionable.

superseded-by: 5402

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/3379-implement-moduleResolutionPackageIdWithRelativeAndAbsolutePath.md` before this move
- `issues/done/3379-implement-moduleResolutionPackageIdWithRelativeAndAbsolutePath.md` after this move

Split follow-up: none created; existing issue `5402` is the tracking item for
the current first blocker.
