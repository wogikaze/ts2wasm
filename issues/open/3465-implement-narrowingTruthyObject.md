---
id: 3465
title: "Implement Narrowingtruthyobject"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed after splitting the current nullable-object receiver blocker to
`issues/open/5455-report-nullable-object-receiver-after-typeof-object-check.md`.

## Problem

Reference test results show 1 case fails in directory
`narrowingTruthyObject` with diagnostics: object-literal.

Fresh triage shows the current blocker is not object-literal parsing. The
source parses and resolves, then lower_program reports issue-211 for
`x.toString()` after `typeof x === 'object'`.

TypeScript reports TS18047 at that same receiver because `x` is still possibly
null. The current compiler should be driven by that nullable-receiver behavior
before later truthy-object narrowing is evaluated.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts --detail
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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts
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

- [x] `issues/open/5455-report-nullable-object-receiver-after-typeof-object-check.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts`

## Duplicate detection

- `issues/open/5451-classify-number-tostring-after-typeof-switch-narrowing.md`
  is related but no-match: it owns number `toString(radix)` after a
  `typeof` switch branch.
- `issues/open/5222-support-interface-typed-method-calls-on-erased-locals.md`
  is related but no-match: it owns interface-typed erased-local method calls.
- `issues/open/342-implement-object-builtin-coverage.md` is a broad Object
  builtin umbrella and not a narrow owner for this nullable receiver.
- Generic method-call and object-literal buckets are no-match because they do
  not capture the TypeScript TS18047 possibly-null receiver behavior.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts
```

Result:

```text
Feature label: object-literal
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: issue-211: unknown receiver class for method `toString` at 122..134
Failure location: line 6, column 14
Source context: x.toString();
tokens: ok
ast: ok; first if condition is typeof x === "object"
resolved: ok through builtins
lower_program: issue-211 unknown receiver class for method `toString`
TypeScript oracle: TS18047 "'x' is possibly 'null'."
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=object-literal:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedSyntax, object-literal
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingTruthyObject.ts
result: pass; current blocker is nullable object receiver before method-call lowering, split to issue 5455
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After 5455 is implemented, this reference may expose truthiness-guarded
  object method receiver support for `x.toString()` or
  `x.hasOwnProperty("x")`. Split those separately if needed.
