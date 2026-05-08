---
id: 3446
title: "Implement Narrowrefinedconstlikeparameterbindingelementnameininnerscope"
type: spike
area: frontend/resolver
class: blocked
priority: P2
depends_on: [5006]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed as superseded by `issues/open/5169-parse-asi-after-expression-statement.md`.
Fresh triage shows this bucket stops at an expression-statement ASI parser
boundary before a closing block, not at name resolution.

## Problem

Reference test results show 1 case failing in directory
`narrowRefinedConstLikeParameterBIndingElementNameInInnerScope` with
diagnostics: scope-analysis. Fresh evidence shows the observable blocker is a
parser syntax failure after `b = () => { ... }` when the next token is the
closing `}` of the containing block.

Problem: narrowRefinedConstLikeParameterBIndingElementNameInInnerScope has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with existing expression-statement ASI owner
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.ts
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

- [x] `issues/open/5169-parse-asi-after-expression-statement.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.ts`

## Duplicate detection

- Superseded by `issues/open/5169-parse-asi-after-expression-statement.md`,
  which owns ASI after completed expression statements. This bucket adds the
  closing-block variant after an arrow-function assignment expression.

No-match candidates:

- `issues/open/5210-parse-do-while-asi-before-block-end-or-expression.md`
  owns the special `do ... while (...)` optional semicolon boundary, not a
  general expression statement.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=scope-analysis:1
```

Fresh triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.ts

result:
Feature label: scope-analysis
Diagnostic code: UnsupportedSyntax
Message: expected Semicolon, got Some(RightBrace) at 196..197
Failure line 10, column 3:
  }
```

Compiler evidence:

```text
tokens: ok
ast: fails after parsing `b = () => { const x: string = a; }` as a completed expression statement without an explicit semicolon
visible symbols: ff, x
TypeScript oracle: ok, diagnostics=[]
TypeScript AST path: SourceFile -> FunctionDeclaration -> Block -> IfStatement -> Block
```

## Completion evidence

Closed as superseded by `issues/open/5169-parse-asi-after-expression-statement.md`;
no new child issue created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, UnsupportedSyntax at expression-statement ASI before RightBrace
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowRefinedConstLikeParameterBIndingElementNameInInnerScope.ts
result: pass; reproduced `expected Semicolon, got Some(RightBrace)` after `b = () => { ... }`
date: 2026-05-08
```

Remaining risks:

- After issue 5169 accepts this ASI boundary, this path may expose the original
  narrowing behavior for destructured const-like parameters captured in an
  inner arrow function.
