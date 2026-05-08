---
id: 3525
title: "Implement Nocrashonnolib"
type: spike
area: ir/lowering
class: superseded
priority: P1
depends_on: [5470]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5470-support-array-spread-over-array-fallback-expressions.md`. Fresh triage shows the current blocker is issue-274 array literal spread over the fallback expression `e || []`.

## Problem

Reference test results originally showed 1 case failing in directory `noCrashOnNoLib` with diagnostics: import-export. Fresh evidence now reaches lowering and reports the spread boundary:

```text
UnsupportedSyntax: issue-274: array literal spread is only supported for literal arrays and ASCII literal-derived strings in this milestone
```

Problem: this generated bucket is not a standalone implementation order. The current observable spread blocker is split to issue 5470.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnNoLib.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnNoLib.ts --detail --no-dashboard-data
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Compiler evidence:

```text
tokens: ok through export function, typed let e, while loop, and `[...(e || [])]`
ast: ok; array literal contains Spread(Binary(Ident e, Or, Array []))
resolved: ok through builtins
lower_program: issue-274 array literal spread boundary
```

TypeScript oracle:

```text
ok; diagnostics=[]
hint: e has type {}[]
```

## Desired final state

This generated bucket is closed as superseded after splitting the current spread blocker to issue 5470. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage evidence
- [x] Confirm existing spread issues do not exactly cover `e || []` fallback operands
- [x] Split the array fallback spread family to issue 5470
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

- [x] Duplicate candidates below are confirmed as split/superseded
- [x] Child issue 5470 contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnNoLib.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnNoLib.ts
```

Not run:

- `cargo fmt --all --check` / `cargo nextest run`: issue metadata-only split; no Rust implementation changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5470-support-array-spread-over-array-fallback-expressions.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noCrashOnNoLib.ts`

## Duplicate detection

- `issues/open/274-implement-spread-operator.md`: broad blocked spread meta issue, not an implementation-ready owner for this slice.
- `issues/open/5456-support-array-spread-over-narrowed-array-typed-parameters.md`: related array spread issue, but it covers narrowed function parameters rather than local fallback expressions.
- No exact existing implementation-ready owner was found for `[...(e || [])]`, so issue 5470 was created.

## Smart triage

Generated on 2026-05-08.

```text
Feature label: spread
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: issue-274: array literal spread is only supported for literal arrays and ASCII literal-derived strings in this milestone
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
