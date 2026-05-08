---
id: 3454
title: "Implement Narrowingconstrainedtypeparameter"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
status: done
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed as superseded by
`issues/open/5450-support-function-typed-parameter-local-calls.md`.

Fresh smart triage shows the current representative blocker is the direct
function-typed parameter call `voice(pet)`, which matches issue 5450's scoped
`function f(g: () => number) { return g(); }` family.

## Problem

Reference test results originally showed 1 case failing in directory
`narrowingConstrainedTypeParameter` with stale diagnostics: import-export.
Fresh triage reaches lowering and reports a generic issue-211 function-valued
local call diagnostic for a direct function-typed parameter call:

```text
UnsupportedSyntax: issue-211: function-valued local calls such as extracted method `voice(...)` are not supported; call receiver.method(...) directly at 394..404
```

This is not a distinct narrowing-constrained-type-parameter implementation
slice. It is blocked before the narrowing behavior by the existing
function-typed parameter local call family.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingConstrainedTypeParameter.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingConstrainedTypeParameter.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowingConstrainedTypeParameter.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowingConstrainedTypeParameter.ts
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

- [x] superseded by `issues/open/5450-support-function-typed-parameter-local-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowingConstrainedTypeParameter.ts`

## Duplicate detection

- `issues/open/5450-support-function-typed-parameter-local-calls.md` is a
  match: this case fails at `voice(pet)`, where `voice` is a parameter typed
  `(pet: TPet) => string`.
- `issues/open/5279-report-function-typed-local-call-definite-assignment.md`
  is no-match: it covers uninitialized function-typed locals and TS2454, while
  this case has an initialized function parameter and TypeScript diagnostics
  are empty.
- Broad method-call buckets such as 435 and 2907 are no-match for this
  closure because 5450 is the narrower existing owner.

## Smart triage

Generated on 2026-05-08 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingConstrainedTypeParameter.ts
```

Result:

```text
Feature label: method-call
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: issue-211: function-valued local calls such as extracted method `voice(...)` are not supported; call receiver.method(...) directly at 394..404
Failure location: line 19, column 2
Source context: return voice(pet);
tokens: ok
ast: ok; ExportDecl Function speak, If, Throw, Return Call callee Ident voice
resolved/lowered: issue-211 at voice(pet)
TypeScript oracle: ok, diagnostics=[]
```

Coverage evidence:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingConstrainedTypeParameter.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `8759dc62d` created the superseding issue 5450 in the same blocker cleanup wave

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowingConstrainedTypeParameter.ts --detail --no-dashboard-data
result: pass; reproduced unsupported=1, UnsupportedSyntax, unknown-unsupported
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowingConstrainedTypeParameter.ts
result: pass; reproduced issue-211 at voice(pet), superseded by issue 5450
date: 2026-05-08

command: cargo fmt --all --check
result: not run; no Rust changes
date: 2026-05-08

command: cargo nextest run
result: not run; no Rust changes
date: 2026-05-08
```

Remaining risks:

- After issue 5450 is implemented, this reference may expose later constrained
  generic narrowing behavior. That should be captured as a new narrower issue
  if it appears.
