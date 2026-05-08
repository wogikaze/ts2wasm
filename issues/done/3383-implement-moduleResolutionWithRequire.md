---
id: 3383
title: "Implement Moduleresolutionwithrequire (audit reopened #3383)"
type: maintenance
area: compiler/module-graph
class: superseded
priority: P1
depends_on: [432, 5425]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage moduleResolutionWithRequire across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this generated bucket reaches AST and module-graph/lowering
work, then fails lowered IR validation for a local CommonJS `require` between
virtual `@filename` sections.

Problem: `moduleResolutionWithRequire.ts` lowers `require("./other")` to a
`ModuleLoad` whose `module_id` is absent from the program module list.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithRequire.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithRequire.ts --detail --no-dashboard-data
```

Observed result:

```text
UnsupportedModule: ModuleLoad references module_id 1 which is not in the program's module list
coverage: unsupported_diagcodes=UnsupportedModule:1, unsupported_features=import-export:1
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5425a-resolve-local-require-between-filename-sections.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one focused implementation-ready child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Child issue contains an exact reference-triage command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

Issue-only close; Rust gates were not required for this lifecycle split.

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithRequire.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithRequire.ts
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

- [x] `issues/open/5425a-resolve-local-require-between-filename-sections.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionWithRequire.ts`

## Duplicate detection

- `issues/open/5229a-resolve-imports-between-filename-sections.md` covers
  static import/export source specifiers between virtual sections, not
  CommonJS `require("./other")` lowered `ModuleLoad` validation.
- `issues/open/5295-resolve-import-equals-require-to-virtual-node-modules-class-export.md`
  covers `import alias = require("pkg")` through virtual node_modules, not a
  local call expression `const { other } = require("./other")`.
- No exact owner existed for the local `require` ModuleLoad invariant. Split
  to issue 5425.

## Smart triage

Fresh triage:

```text
feature: invariant-violation
diagnostic: InvariantViolation / compiler-invariant
visible symbols: other, require, foo, a
ast: ok; ExportDecl `other`, Function `foo`, Let `a = require("../outside-of-rootdir/foo")`, Let `{other} = require("./other")`
resolved/lowered: UnsupportedModule ModuleLoad references module_id 1 which is not in the program's module list
TypeScript oracle: ok with no diagnostics
```

## Completion evidence

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithRequire.ts --detail --no-dashboard-data
result: pass; one UnsupportedModule/import-export path reproduced
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithRequire.ts
result: pass; AST succeeds and lowered validation fails on missing ModuleLoad module_id, split to issue 5425
date: 2026-05-08
```

Remaining risks:

- After issue 5425 lands, the same reference may expose unsupported
  out-of-root `require("../outside-of-rootdir/foo")` behavior or destructuring
  type-checking differences.
## Close note

Superseded by issue 5425, which owns local CommonJS `require` calls between
virtual `@Filename` sections producing invalid `ModuleLoad` references.

superseded-by: 5425

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/3383-implement-moduleResolutionWithRequire.md` before this move
- `issues/done/3383-implement-moduleResolutionWithRequire.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
