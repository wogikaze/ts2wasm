---
id: 3401
title: "Close multiCallOverloads bucket to nested default-parameter closure owner"
type: maintenance
area: ir/lowering
class: superseded
priority: P1
depends_on: [5000, 5389]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated parser-syntax bucket as superseded by issue 5389. Fresh
smart triage shows overload/interface syntax is no longer the first blocker:
the file reaches lowering and stops at the existing issue-062e guard for nested
function expressions with optional/default parameters.

## Problem

The original bucket listed one `multiCallOverloads` reference file under
`parser-syntax` without smart-triage evidence.

Fresh focused coverage reports:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Smart triage classifies the precise current blocker as:

```text
UnsupportedRuntimeSubset: issue-062e: nested function `` closure parameters with defaults or rest are not supported in this slice
```

## Current failure

Representative source shape:

```ts
interface ICallback {
    (x?: string): void;
}

function load(f: ICallback) {}

var f1: ICallback = function(z?) {}
var f2: ICallback = function(z?) {}
load(f1);
load(f2);
load(function() {});
load(function(z?) {});
```

Compiler evidence:

```text
tokens: ok through interface call signature, typed function parameter, and optional function-expression parameters
ast: ok; function(z?) is represented as a FunctionExpr parameter defaulted to Undefined
resolved/lowered: UnsupportedRuntimeSubset issue-062e for nested function closure parameters with defaults or rest
TypeScript oracle: accepts the file with no diagnostics
```

## Desired final state

This generated bucket remains closed. The nested function optional/default
parameter lowering blocker is owned by
`issues/open/5389-support-nested-function-default-parameters-in-closure-lowering.md`.

## Scope

Completed:

- [x] Re-ran focused coverage for the affected reference file.
- [x] Re-ran smart triage for the affected reference file.
- [x] Confirmed parser-syntax is not the current first blocker.
- [x] Confirmed the current blocker matches issue 5389.
- [x] Added an ownership note to issue 5389.

Out of scope:

- Direct implementation from this generated bucket.
- Runtime support for rest parameters, `this`, `arguments`, or mutable captured
  closure environments.
- Full overload resolution or contextual typing semantics after lowering
  advances.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/multiCallOverloads.ts`

## Acceptance criteria

- [x] Current first diagnostic state is recorded.
- [x] Matching owner issue 5389 is identified.
- [x] This bucket is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiCallOverloads.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiCallOverloads.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Issue 5389 already tracks nested ordinary function expressions with optional or
default parameters. This file adds anonymous function-expression representatives
that have optional parameters and are passed through an interface call-signature
shape before reaching the same issue-062e lowering guard.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: focused coverage and triage listed above
result: pass; first precise blocker is issue-062e nested function optional/default parameter closure lowering
date: 2026-05-08
```

Remaining risks:

- After issue 5389 lands, this file may expose contextual call overload
  semantics or runtime behavior gaps.
