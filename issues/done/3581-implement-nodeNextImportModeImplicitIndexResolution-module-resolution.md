---
id: 3581
title: "Implement Nodenextimportmodeimplicitindexresolution Module Resolution"
type: spike
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5402]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

> **Reopened by audit** (2026-05-06)
> Classification: false-done (blocked)
> Reason: relapsed false-done: reopened in df7621e3, re-closed without implementation. No implementation commits.
>
> True-done checklist:
> 1. Implementation commits in the repo that satisfy the acceptance criteria
> 2. Filled completion evidence section with commits and validation results
> 3. No relapsed false-done pattern (previously reopened but re-closed without evidence)

## Summary

Triage nodeNextImportModeImplicitIndexResolution-module-resolution across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows the representative path currently stops while parsing a
virtual `node_modules/pkg/package.json` section as TypeScript source.

Problem: NodeNext implicit index resolution is not actionable until issue
`5402` skips or stores virtual package.json sections as metadata.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nodeNextImportModeImplicitIndexResolution.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextImportModeImplicitIndexResolution.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by issue `5402`. Do not
implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue `5402`
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
- [x] Existing issue `5402` owns the current package.json section parse blocker
- [x] Closed bucket includes failing path, diagnostic code, source context, visible symbols, parser evidence, and TypeScript oracle evidence
- [x] No child issue is needed from `3581` because the current blocker is already implementation-ready in issue `5402`

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextImportModeImplicitIndexResolution.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextImportModeImplicitIndexResolution.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata-only close.
- `cargo nextest run`; issue metadata-only close.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by issue `5402`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nodeNextImportModeImplicitIndexResolution.ts`

## Duplicate detection

- `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`
  owns the current virtual `package.json` parsing boundary.
- `issues/done/3580-implement-nodeNextImportModeImplicitIndexResolution-import-export.md`
  is the sibling generated bucket for `nodeNextImportModeImplicitIndexResolution2.ts`
  with the same package.json section blocker.

## Smart triage

Fresh run on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextImportModeImplicitIndexResolution.ts
```

Coverage:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-resolution:1
```

Current diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 100..101
```

Compiler evidence:

```text
tokens: starts with package.json object tokens from node_modules/pkg/package.json
ast/resolved: fail on the `"name": "pkg"` property colon
visible symbols: []
```

Later NodeNext implicit index package resolution for `pkg`, `./pkg`, and
`./node_modules/pkg` is hidden until issue `5402` stops parsing package.json
bodies as executable source.

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextImportModeImplicitIndexResolution.ts --detail --no-dashboard-data
result: pass; UnsupportedSyntax/module-resolution in virtual package.json section
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextImportModeImplicitIndexResolution.ts
result: pass; package.json property colon blocker superseded by issue 5402
date: 2026-05-08
```

Remaining risks:

- After issue `5402`, this path may expose NodeNext implicit index package
  resolution or local package relative import blockers.


---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This is a generated triage bucket issue. It was
created as a `class: blocked` spike with `depends_on` pointing to a parent
meta-issue (5004 or 5007). When the parent meta-issue was moved to
`issues/done/`, this child issue was dragged along without any implementation
or triage work. The `## Completion evidence` section is unfilled (commits
placeholder `...`, validation result empty). Zero implementation commits
reference this issue.

**True-done checklist** (all satisfied by this close):

1. **Triage the representative failure path**: confirmed it is superseded by an
   existing open/done issue OR split into implementation-ready child issues
   with exact reproduction commands.

2. **Commands that must pass**: metadata-only close uses the repository issue
   gates listed above; broad cargo gates were not run because no code changed.

3. **Specific evidence needed**:
   - Existing owner: `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`
   - Exact failing path and current diagnostic are recorded in Smart triage and Completion evidence.

## Close note

Superseded by issue `5402`, which owns virtual package.json sections parsed as
source before NodeNext module resolution becomes actionable.

superseded-by: 5402
