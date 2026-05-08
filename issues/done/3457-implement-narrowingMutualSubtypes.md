---
id: 3457
title: "Implement Narrowingmutualsubtypes"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as superseded by
`issues/open/5193-parse-asi-after-ambient-variable-declarations.md`.

Fresh smart triage shows the current blocker is issue-400 ASI handling after a
declaration-only ambient variable declaration, not mutual-subtype narrowing.

## Problem

Reference test results show 1 case fails in directory
`narrowingMutualSubtypes` with diagnostics: parser-syntax.

The current blocker is a declaration-only ambient variable without an explicit
semicolon before later runtime declarations:

```ts
declare const r3: { [x: string]: unknown }

const a3a = [c3, r3];
const a3b = [r3, c3];
```

The parser carries the ambient declaration too far and reports an issue-400
ambient initializer diagnostic when it reaches the later initializer.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingMutualSubtypes.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingMutualSubtypes.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingMutualSubtypes.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingMutualSubtypes.ts
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

- [x] superseded by `issues/open/5193-parse-asi-after-ambient-variable-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingMutualSubtypes.ts`

## Duplicate detection

- `issues/open/5193-parse-asi-after-ambient-variable-declarations.md` is a
  match: it owns ASI after declaration-only ambient variables and explicitly
  includes the same issue-400 ambient-initializer false positive family.
- `issues/open/5345-parse-generic-ambient-const-type-annotations.md` is
  related but no-match: this case uses a simple index-signature object type,
  not a generic ambient const type.
- `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`
  is later-stage no-match: this file does not reach name resolution because
  parser ASI fails first.
- `issues/open/059-implement-parser-syntax-extensions.md` is the broad parser
  epic and should not be selected directly when 5193 is the narrower owner.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingMutualSubtypes.ts
```

Result:

```text
Feature label: parser-syntax
Diagnostic: UnsupportedTypeScriptSyntax / unsupported-feature-boundary
Message: issue-400: ambient variable declarations with initializers would affect runtime bindings at 628..629
Failure location: line 24, column 2
Source context: const a3b = [r3, c3]; after semicolonless declare const r3
tokens: ok
ast: fails before AST construction with issue-400
resolved: same parser diagnostic
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingMutualSubtypes.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
unsupported_features=parser-syntax:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingMutualSubtypes.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedTypeScriptSyntax, parser-syntax
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingMutualSubtypes.ts
result: pass; reproduced issue-400 ambient variable ASI blocker, superseded by issue 5193
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After issue 5193 is implemented, this reference may expose ambient value
  name-resolution or mutual-subtype narrowing behavior; split that separately
  if needed.
