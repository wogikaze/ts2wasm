---
id: 3437
title: "Implement Narrowbybooleancomparison"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-06
status: done
---

## Summary

Closed as superseded by
`issues/open/5269-parse-optional-class-property-declarations.md`.

Fresh focused coverage and triage show `narrowByBooleanComparison.ts`
currently fails at the optional class property declaration
`status?: number;`. Existing issue 5269 already owns parsing
TypeScript optional class property declarations such as this construct.

## Problem

Reference test results show 1 case fails in directory
`narrowByBooleanComparison` with diagnostics: parser-syntax. The compiler
cannot handle these syntax/semantics, preventing compilation of code in this
category.

Problem: narrowByBooleanComparison had 1 generated reference failure and
needed smart-triage evidence before implementation starts.

Disposition: no new child issue created because the current first blocker is
already covered by existing implementation-ready issue 5269.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as superseded by an existing implementation-ready owner issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Fresh evidence contains an exact `reference-triage` command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing owner issue 5269 acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts
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

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts`

## Duplicate detection

- `issues/open/5269-parse-optional-class-property-declarations.md`
  owns the current first parser blocker: optional class property declarations
  such as `status?: number;`.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts: UnsupportedSyntax: unknown-unsupported
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts

result:
UnsupportedSyntax: expected LeftParen, got Some(Question) at 1079..1080
line 64, column 11
feature_label: parser-syntax
```

Source context:

```ts
class WebError extends URIError {
    status?: number;
}
```

Compiler evidence:

```text
tokens: ok through the file prefix; the failing class member token stream reaches Question after Ident("status")
ast: fails with expected LeftParen, got Some(Question)
resolved: same parser failure
visible symbols before failure: isA, test1, test2, test3, WebError
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
AST includes ClassDeclaration for WebError and later functions test4-test7
```

## Triage evidence

Date: 2026-05-06

Command:

```sh
python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts
```

Result: superseded by issue 5269. The representative failure is parser syntax
at `status?: number;` inside a class property:

```text
UnsupportedSyntax: expected LeftParen, got Some(Question) at 1079..1080
feature_label: parser-syntax
```

No new child issue was created in this pass because issue 5269 already owns
the exact parser feature.

Remaining risks:

- none

## Completion evidence

### Issue cleanup commits

- `...`

### Changed files

- `issues/done/3437-implement-narrowByBooleanComparison.md`
- `issues/open/5269-parse-optional-class-property-declarations.md`
- `issues/index.md`

### Validation

```sh
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax at optional class property declaration
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByBooleanComparison.ts
result: pass; current first blocker is `expected LeftParen, got Some(Question)` for `status?: number;`, owned by issue 5269
date: 2026-05-08
```
