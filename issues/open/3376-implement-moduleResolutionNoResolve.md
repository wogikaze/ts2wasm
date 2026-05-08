---
id: 3376
title: "Implement Moduleresolutionnoresolve (audit reopened #3376)"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432, 5285]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage moduleResolutionNoResolve across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case in `moduleResolutionNoResolve`. Fresh
triage confirms the current first blocker is not `noResolve` policy itself:
the second virtual file section contains `export var c = '';`, and the frontend
stops at the generic issue-055 initialized variable export boundary.

Problem: this generated bucket is superseded by existing initialized
`export var` issue `5285`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolutionNoResolve.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionNoResolve.ts --detail
```

## Desired final state

This generated bucket is closed. The actionable first blocker is tracked by
issue `5285`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede with existing implementation-ready issue `5285`
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
- [x] Superseding issue `5285` contains exact `reference-triage` ownership evidence
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionNoResolve.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolutionNoResolve.ts
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

- [x] none; superseded by existing issue `5285`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionNoResolve.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated manually on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-coverage tsc \
  --path-filter reference/typescript/tests/cases/compiler/moduleResolutionNoResolve.ts \
  --detail --no-dashboard-data
```

Result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-resolution:1
per-file: moduleResolutionNoResolve.ts => UnsupportedSyntax / module-resolution
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/moduleResolutionNoResolve.ts
```

Observed:

```text
diagnosis: UnsupportedModule / unsupported-feature-boundary
message: issue-055: unsupported variable export; module resolution and loading are not implemented at 0..6
actual AST/resolved error: issue-055 unsupported variable export at 134..140
source:
// @filename: a.ts
import a = require('./b');

// @filename: b.ts
export var c = '';
```

Compiler evidence:

```text
tokens: ok through ImportEqualsDeclaration tokens and Export Var Ident("c") = String("") ;
ast/resolved: fail at initialized export var before noResolve module lookup behavior
TypeScript oracle: ImportEqualsDeclaration, FirstStatement "export var c = '';",
and TS2307 for './b' because --noResolve intentionally avoids resolving b.ts
```

Existing issue `5285` exactly covers initialized `export var name = expr;`
declarations that currently stop at the generic issue-055 variable export
boundary.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionNoResolve.ts --detail --no-dashboard-data
result: pass; reproduced UnsupportedSyntax/module-resolution first blocker
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionNoResolve.ts
result: pass; first actionable blocker is initialized export var parsing, superseded by issue 5285
date: 2026-05-08
```

Remaining risks:

- After issue `5285`, this reference should advance to `--noResolve` relative
  import diagnostics for `./b`, or another narrower import-equals/module
  resolution blocker.

## Close note

Superseded by issue `5285`. Fresh triage stops at the second virtual section's
initialized `export var c = '';` declaration before `--noResolve` behavior is
actionable.

superseded-by: 5285

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/3376-implement-moduleResolutionNoResolve.md` before this move
- `issues/open/3376-implement-moduleResolutionNoResolve.md` after this move

Split follow-up: none created; existing issue `5285` is the tracking item for
the current first blocker.
