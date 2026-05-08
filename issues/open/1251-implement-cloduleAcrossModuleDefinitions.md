---
id: 1251
title: "Implement Cloduleacrossmoduledefinitions"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1251.

## Summary

Closed as stale. Fresh triage and focused coverage show
`reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts`
now build-passes, so there is no current import/export blocker to split into a
child issue.

## Problem

Reference test results previously showed 1 case failing in directory
`cloduleAcrossModuleDefinitions` with diagnostics: import-export. Fresh triage
shows tokenization, AST, resolution, and build all succeed.

Problem: the generated import/export bucket is stale. The representative
reference now build-passes and TypeScript reports no diagnostics.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as stale because the representative path now
reports `build_pass`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the representative case is now build-pass with no active import/export blocker
- [x] Close as stale build-pass instead of creating a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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

- [x] Duplicate candidates below are confirmed; no separate issue is needed
- [x] No child issue needed because the representative case now build-passes
- [x] This issue includes path, diagnostic status, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and build-pass result

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current compiler build has no import/export blocker on this path

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts`

## Duplicate detection

No exact implementation child is created because the original import/export
blocker no longer reproduces.

Resolution:

```text
The original import/export blocker is stale. The reference window now reports build_pass, and the TypeScript oracle reports no diagnostics.
```

## Smart triage

### Smart triage: Build pass: cloduleAcrossModuleDefinitions

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
unsupported_features=
```

Source context:

```ts
namespace A {
    export class B {
        foo() { }
        static bar() { }
    }
}

namespace A {
    export namespace B {
        export var x = 1;
    }
}

var b: A.B;
```

Compiler evidence:

```text
tokens: ok; namespace A, export class B, export namespace B, export var x, and var b are tokenized
ast: ok; retained AST contains `var b` with TypeScript namespace/class declarations erased
resolved: ok; retained `b` binding resolves
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
binding hints: x is number; b is B
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; build succeeded and original import/export blocker is stale
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleAcrossModuleDefinitions.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; executed=1, build_pass=1, unsupported=0
date: 2026-05-07
```

Remaining risks:

- none; TypeScript oracle reports no diagnostics for this reference
