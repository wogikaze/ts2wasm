---
id: 3453
title: "Implement Narrowingbytypeofinswitch"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed after split to `issues/open/5451-classify-number-tostring-after-typeof-switch-narrowing.md`.
Fresh triage shows this bucket parses and resolves, then stops at `x.toString(2)`
inside a `case 'number'` branch.

## Problem

Reference test results show 1 case failing in directory
`narrowingByTypeofInSwitch` with diagnostics: class. Fresh evidence shows the
current blocker is the issue-211 unknown receiver diagnostic for
`x.toString(2)` after `typeof` switch narrowing.

Problem: narrowingByTypeofInSwitch has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split to a narrow number `toString` after `typeof` switch narrowing issue
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts
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

- [x] `issues/open/5451-classify-number-tostring-after-typeof-switch-narrowing.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts`

## Duplicate detection

- No exact existing owner found. `issues/open/5383-classify-number-parameter-tofixed-calls.md`
  is related but limited to `toFixed()` on number-annotated arrow parameters,
  not `typeof` switch narrowing followed by `toString(2)`.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts --detail --no-dashboard-data

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts

result:
Feature label: class
Diagnostic code: UnsupportedSyntax
Message: issue-211: unknown receiver class for method `toString` at 3599..3612
Failure line 143, column 31:
        case 'number': return x.toString(2);
```

Compiler evidence:

```text
tokens: ok
ast: ok; switch typeof cases and return member call parse
resolved: ok through builtins
lower_program: issue-211 unknown receiver class for method `toString`
visible symbols: assertNever/assertNumber/.../exhaustiveChecks
TypeScript oracle: ok, diagnostics=[]
```

## Completion evidence

Closed after split to `issues/open/5451-classify-number-tostring-after-typeof-switch-narrowing.md`.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, issue-211 at number-narrowed `toString`
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingByTypeofInSwitch.ts
result: pass; parser/resolver ok, reproduced issue-211 at `x.toString(2)`
date: 2026-05-08
```

Remaining risks:

- After issue 5451 advances past `x.toString(2)`, this path may expose later
  callable-union or object-property narrowing blockers.
