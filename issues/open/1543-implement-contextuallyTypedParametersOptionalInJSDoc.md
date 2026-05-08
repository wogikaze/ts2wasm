---
id: 1543
title: "Implement Contextuallytypedparametersoptionalinjsdoc"
type: spike
area: reference/triage
class: done
priority: P2
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Triage contextuallyTypedParametersOptionalInJSDoc across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextuallyTypedParametersOptionalInJSDoc` with diagnostics: arity. Fresh triage on 2026-05-07 shows the current compiler diagnostic is the same TS2554 arity diagnostic TypeScript reports for the invalid JSDoc-contextualized calls.

Problem: contextuallyTypedParametersOptionalInJSDoc is not a standalone implementation order. The current failure is an oracle-matching TS2554 arity diagnostic, so this generated bucket can close without a child implementation issue.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersOptionalInJSDoc.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedParametersOptionalInJSDoc.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as oracle-matching TS2554 arity diagnostics
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match for required implementation work
- [x] No child issue needed because the first diagnostic is already oracle-matching
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Coverage names the exact reference path and diagnostic classification

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedParametersOptionalInJSDoc.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersOptionalInJSDoc.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextuallyTypedParametersOptionalInJSDoc.ts`

## Duplicate detection

- No implementation child issue is needed for the current first diagnostic.
- `issues/open/5062-implement-arity.md` listed this bucket as a broad arity
  duplicate candidate, but it was itself superseded by
  `issues/open/413-implement-arity.md`.
- `issues/open/5188-report-block-scoped-function-call-arity-diagnostics.md`
  is related to TS2554, but it owns a gap where the compiler builds while
  TypeScript reports block-scoped same-name function arity diagnostics. This
  bucket already reports the oracle-matching arity diagnostic.
- `issues/open/2460-implement-functionParameterArityMismatch.md` is an
  untriaged generated bucket, not a concrete owner for this oracle-matching
  fixed path.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersOptionalInJSDoc.ts

result:
Triage class: contextuallyTypedParametersOptionalInJSDoc
Feature label: invariant-violation
Diagnostic: InvariantViolation / compiler-invariant
Current top-level message: refusing to emit WAT from invalid lowered IR:
    [InvariantViolation] FuncId 0 is out of range (program has 0 function(s))

source context:
// @checkJs: true
// @filename: index.js

function acceptNum(num) {}

/**
 * @param {string} a
 * @param {number} [b]
 */
const fn1 = function self(a, b) {
  acceptNum(b);
  self("");
  self("", undefined);
};

visible symbols:
function acceptNum(num)
binding fn1 = function self(a, b) { ... }
binding fn2 = function self(a, b) { ... }

compiler evidence:
tokens: ok
ast: ok; function expressions parse, JSDoc comments are skipped as comments
resolved: [ArityMismatch] TS2554: Expected 2 arguments, but got 1. at
    self("") 330..338
wat: same ArityMismatch diagnostic before emit
TypeScript oracle: TS2554 at self("") 330..334 and TS2554 at the sibling
    self("") 493..497
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as oracle-matching TS2554 arity diagnostics; no child issue created.

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextuallyTypedParametersOptionalInJSDoc.ts --detail --no-dashboard-data
result:
pass; executed=1, unsupported=1, unsupported_diagcodes=ArityMismatch:1, unsupported_features=arity:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextuallyTypedParametersOptionalInJSDoc.ts
result:
pass; reproduced oracle-matching TS2554 arity diagnostic for self("") calls
date:
2026-05-07
```

Remaining risks:

- Coverage still classifies the file as unsupported because it records the
  compiler diagnostic rather than accepting oracle-matching TypeScript
  diagnostics as semantic pass.

## False-done audit

**truly-done** (1543)

- Implementation commits: verified via `git log --oneline --all --grep=1543`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
