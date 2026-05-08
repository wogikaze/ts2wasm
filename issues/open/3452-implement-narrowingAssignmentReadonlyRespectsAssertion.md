---
id: 3452
title: "Implement Narrowingassignmentreadonlyrespectsassertion"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [059]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
---

## Summary

Closed after split to `issues/open/5450-support-function-typed-parameter-local-calls.md`.
Fresh triage shows this bucket parses and resolves, then stops at a direct
function-typed parameter call `subFunc()`.

## Problem

Reference test results show 1 case failing in directory
`narrowingAssignmentReadonlyRespectsAssertion` with diagnostics:
type-assertion. Fresh evidence shows the current blocker is not readonly
narrowing yet: lower_program reports issue-211 for the function-typed
parameter call `subFunc()`.

Problem: narrowingAssignmentReadonlyRespectsAssertion has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts
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

- [x] `issues/open/5450-support-function-typed-parameter-local-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts`

## Duplicate detection

- No exact existing owner found. Related callable-local issues are no-match:
  `5195` covers callable interface-typed locals, `5196` covers conditional
  type callable parameters, `5374` covers ambient callable const locals, and
  `5440` covers initialized function-expression locals.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=type-assertion:1
```

Fresh triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts

result:
Feature label: type-assertion
Diagnostic code: UnsupportedSyntax
Message: issue-211: function-valued local calls such as extracted method `subFunc(...)` are not supported; call receiver.method(...) directly at 584..593
Failure line 23, column 19:
  return { cases: subFunc() };
```

Compiler evidence:

```text
tokens: ok
ast: ok; interfaces, generic functions, object literal return, and call expression parse
resolved: ok through builtins
lower_program: issue-211 at `subFunc()`
visible symbols: subDataFunc
TypeScript oracle: ok, diagnostics=[]
TypeScript AST path: SourceFile -> FunctionDeclaration -> Block -> ReturnStatement -> ObjectLiteralExpression -> PropertyAssignment -> CallExpression -> Identifier subFunc
```

## Triage evidence

Date: 2026-05-06

Command:

```sh
python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts
```

Result: still open. The representative failure is a function-valued local call:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `subFunc(...)` are not supported
feature_label: type-assertion
```

No implementation-ready child was created in this pass; this bucket still needs semantic/runtime triage rather than closure.

Remaining risks:

- none

## Completion evidence

Closed after split to `issues/open/5450-support-function-typed-parameter-local-calls.md`.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, issue-211 at function-typed parameter call
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingAssignmentReadonlyRespectsAssertion.ts
result: pass; parser/resolver ok, reproduced issue-211 at `subFunc()`
date: 2026-05-08
```

Remaining risks:

- After issue 5450 advances past `subFunc()`, this path may expose readonly
  array assertion/narrowing behavior.
