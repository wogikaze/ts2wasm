---
id: 3447
title: "Implement Narrowswitchoptionalchaincontainmentevolvingarraynocrash"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed as superseded by `issues/open/5445-parse-braced-switch-case-clause-statements.md`.
Fresh triage shows the representative path reaches the same braced switch case
body parser blocker before optional-chain containment or evolving-array
semantics.

## Problem

Reference test results show 1 case failing in directory
`narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash` with diagnostics:
parser-syntax. Fresh evidence shows the observable blocker is parsing
`bar.push("baz");` inside a braced `case` clause body.

Problem: narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing braced switch case body parser owner
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the owner issue

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
- [x] Existing owner contains an exact `reference-triage` command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Owner issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash1.ts
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

- [x] `issues/open/5445-parse-braced-switch-case-clause-statements.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash1.ts`

## Duplicate detection

- Superseded by `issues/open/5445-parse-braced-switch-case-clause-statements.md`,
  which owns member-access expression statements inside braced switch case
  clause bodies.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash1.ts --detail --no-dashboard-data

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash1.ts

result:
Feature label: parser-syntax
Diagnostic code: UnsupportedSyntax
Message: expected Comma, got Some(Dot) at 174..175
Failure line 16, column 8:
    bar.push("baz");
```

Compiler evidence:

```text
tokens: ok; OptionalChain tokens for `foo?.length` and `bar?.length` are present
ast: fails inside the braced `case 1: { ... }` body at `bar.push("baz");`
visible symbols: foo, bar
TypeScript oracle: parses the switch/case block and reports later TS2345 for pushing `"baz"` into `never[]`
TypeScript AST path: SourceFile -> SwitchStatement -> CaseBlock -> CaseClause -> Block -> ExpressionStatement -> CallExpression -> PropertyAccessExpression
```

## Completion evidence

Closed as superseded by `issues/open/5445-parse-braced-switch-case-clause-statements.md`;
no new child issue created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash1.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, UnsupportedSyntax at braced case body member call
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowSwitchOptionalChainContainmentEvolvingArrayNoCrash1.ts
result: pass; reproduced `expected Comma, got Some(Dot)` at `bar.push("baz");`
date: 2026-05-08
```

Remaining risks:

- After issue 5445 parses braced case bodies, this path may expose a later
  evolving-array or optional-chain switch containment semantic requirement.
