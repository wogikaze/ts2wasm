---
id: 3469
title: "Implement Narrowinguniontounion"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5001]
blocks: []
status: done
completed: 2026-05-08
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as superseded by the existing implementation-ready generic ambient
const annotation owner:
`issues/open/5345-parse-generic-ambient-const-type-annotations.md`.

## Problem

Reference test results show 1 case fails in directory `narrowingUnionToUnion`
with diagnostics: arrow-function.

Fresh triage shows the current blocker is not arrow-function behavior. The
parser reaches the ambient declarations:

```ts
declare const broken: Record<string, any> | undefined;
declare const workingAgain: Record<string, any> | undefined | unknown;
```

and reports `issue-400: unterminated ambient variable declaration` before the
later discriminated-union narrowing expressions. This is the same generic
ambient const annotation boundary already tracked by issue 5345.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingUnionToUnion.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingUnionToUnion.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingUnionToUnion.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingUnionToUnion.ts
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

- [x] folded into `issues/open/5345-parse-generic-ambient-const-type-annotations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingUnionToUnion.ts`

## Duplicate detection

- `issues/open/5345-parse-generic-ambient-const-type-annotations.md` is an
  exact implementation-ready owner for generic ambient const annotations that
  stop at issue-400 before the later semantic behavior.
- Broad parser syntax issues 059 and 442 are no-match because 5345 is the
  narrow implementation owner.
- Arrow-function issues are no-match because fresh triage reaches a later
  ambient const parser boundary.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingUnionToUnion.ts
```

Result:

```text
Feature label: parser-syntax
Diagnostic: UnsupportedTypeScriptSyntax / unsupported-feature-boundary
Message: issue-400: unterminated ambient variable declaration at 5171..5178
Failure location: line 228, column 23
Relevant declarations:
  declare const broken: Record<string, any> | undefined;
  declare const workingAgain: Record<string, any> | undefined | unknown;
tokens: ok through the declarations and following && expressions
ast: fails at issue-400 before later narrowing expressions
resolved: same parser boundary
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingUnionToUnion.ts --detail --no-dashboard-data
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
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingUnionToUnion.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedTypeScriptSyntax, parser-syntax
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingUnionToUnion.ts
result: pass; current blocker is generic ambient const annotation parsing, folded into issue 5345
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After issue 5345 is implemented, this reference may expose arrow-function,
  assertion-function, discriminated-union, or narrowing behavior. Split those
  separately if they appear.
