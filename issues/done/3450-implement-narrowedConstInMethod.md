---
id: 3450
title: "Implement Narrowedconstinmethod"
type: spike
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed after split to `issues/open/5449-lower-anonymous-class-expressions-in-return-statements.md`.
Fresh triage shows parsing and resolution advance to the `issue-313` class
expression lowering boundary.

## Problem

Reference test results show 1 case failing in directory `narrowedConstInMethod`
with diagnostics: unknown-unsupported. Fresh evidence shows the current blocker
is lowering the anonymous `class { ... }` expression returned from `f2`.

Problem: narrowedConstInMethod has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split to a narrow anonymous-class-expression lowering issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts
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

- [x] `issues/open/5449-lower-anonymous-class-expressions-in-return-statements.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts`

## Duplicate detection

- Split to `issues/open/5449-lower-anonymous-class-expressions-in-return-statements.md`.

Related but distinct:

- `issues/open/5248-lower-class-expressions.md` owns named class expressions
  used as assignment/initializer values, not this returned anonymous class
  expression.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Fresh triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts

result:
Feature label: class
Diagnostic code: UnsupportedSyntax
Message: issue-313: class expression lowering not yet implemented
```

Compiler evidence:

```text
tokens: ok
ast: ok; `f` object-literal method and `f2` returned anonymous class expression parse
resolved: ok through builtins
lower_program: UnsupportedSyntax issue-313 class expression lowering not yet implemented
visible symbols: f, x, f2, x
TypeScript oracle: ok, diagnostics=[]
```

## Completion evidence

Closed after split to `issues/open/5449-lower-anonymous-class-expressions-in-return-statements.md`.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, issue-313 class expression lowering
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowedConstInMethod.ts
result: pass; parser/resolver ok, reproduced issue-313 in lower_program
date: 2026-05-08
```

Remaining risks:

- After issue 5449 lowers returned anonymous class expressions, this path may
  expose captured-const narrowing behavior inside returned methods.
