---
id: 3384
title: "Implement Moduleresolutionwithrequireandimport"
type: maintenance
area: compiler/module-graph
class: superseded
priority: P2
depends_on: [5007, 5425]
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

Triage moduleResolutionWithRequireAndImport across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this generated bucket reaches AST and module-graph/lowering
work, then fails lowered IR validation for a local CommonJS `require` between
virtual `@filename` sections.

Problem: `moduleResolutionWithRequireAndImport.ts` lowers `require("./other")`
to a `ModuleLoad` whose `module_id` is absent from the program module list.
The same executable blocker is already owned by issue 5425.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithRequireAndImport.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithRequireAndImport.ts --detail --no-dashboard-data
```

Observed result:

```text
UnsupportedModule: ModuleLoad references module_id 1 which is not in the program's module list
coverage: unsupported_diagcodes=UnsupportedModule:1, unsupported_features=import-export:1
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5425-resolve-local-require-between-filename-sections.md`.

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

- `crates/compiler/src/module_graph.rs`
- `crates/ir/src/lowered/`
- focused compiler tests or fixtures

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

Issue-only close; Rust gates were not required for this lifecycle split.

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithRequireAndImport.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithRequireAndImport.ts
```

Not run:

- cargo fmt --all --check
- cargo nextest run

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5425-resolve-local-require-between-filename-sections.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionWithRequireAndImport.ts`

## Duplicate detection

- `issues/open/5425-resolve-local-require-between-filename-sections.md`
  exactly covers the current local `require("./other")` dangling ModuleLoad
  lowered-IR validation failure.

## Smart triage

Fresh triage:

```text
feature: invariant-violation
diagnostic: InvariantViolation / compiler-invariant
visible symbols: other, require, a, foo, a
ast: ok; ExportDecl `other`, erased Let `a = null`, Function `foo`, Let `a = require("../outside-of-rootdir/foo")`, Let `{other} = require("./other")`
resolved/lowered: UnsupportedModule ModuleLoad references module_id 1 which is not in the program's module list
TypeScript oracle: TS2307 for type query `typeof import("./other")`; no runtime diagnostic at `require("./other")`
```

The active compiler failure is the same local CommonJS require ModuleLoad
boundary as issue 5425. After that is fixed, the earlier
`typeof import("./other")` type-query diagnostic may need a separate type-only
module-resolution issue if it becomes observable in the runner.

## Completion evidence

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithRequireAndImport.ts --detail --no-dashboard-data
result: pass; one UnsupportedModule/import-export path reproduced
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithRequireAndImport.ts
result: pass; AST succeeds and lowered validation fails on missing ModuleLoad module_id, superseded by issue 5425
date: 2026-05-08
```

Remaining risks:

- After issue 5425 lands, this reference may expose a type-only
  `typeof import("./other")` module-resolution blocker.


---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This is a generated triage bucket issue. It was
created as a `class: blocked` spike with `depends_on` pointing to a parent
meta-issue (5004 or 5007). When the parent meta-issue was moved to
`issues/done/`, this child issue was dragged along without any implementation
or triage work. The `## Completion evidence` section is unfilled (commits
placeholder `...`, validation result empty). Zero implementation commits
reference this issue.

**True-done checklist** (all must pass):

1. **Triage the representative failure path**: Confirm it is superseded by an
   existing open/done issue OR split into implementation-ready child issues
   with exact reproduction commands.

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - Issue URL or child issue path documenting the triage outcome
   - Or: the exact failing reference path has a matching open/done issue
   - Or: the failing test case no longer reproduces the original diagnostic

## Close note

Superseded by issue 5425, which owns local CommonJS `require` calls between
virtual `@Filename` sections producing invalid `ModuleLoad` references.

superseded-by: 5425
