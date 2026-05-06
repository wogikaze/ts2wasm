---
id: 5247
title: "Fix JS noEmit class constructor FuncId invariant"
type: feature
area: ir/compiler
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Fix the lowered-IR/WAT validation path that reports an out-of-range constructor
function id for JS/noEmit class declarations.

## Problem

Problem: `classAttributeInferenceTemplateJS.ts` triage reports `InvariantViolation: ClassDecl constructor FuncId 0 is out of range (program has 0 function(s))`.

Fresh coverage builds the file, but `reference-triage` exposes the invariant
while dumping the JS/noEmit pipeline.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classAttributeInferenceTemplateJS.ts
```

Current diagnostic:

```text
InvariantViolation: ClassDecl constructor FuncId 0 is out of range (program has 0 function(s))
```

## Scope

In scope:

- [ ] Preserve or remove class constructor `FuncId` references consistently when JS/noEmit lowering has zero emitted functions.
- [ ] Add focused validation coverage for a noEmit/checkJs class with a constructor.
- [ ] Confirm the representative no longer reports the invariant.

Out of scope:

- TypeScript type inference for class attributes.
- Broad class runtime semantics unrelated to the constructor function id invariant.

## Affected paths

Expected: `crates/ir/`, `crates/compiler/src/`, `crates/cli/tests/`, `fixtures/`.

Do not touch: module export semantics or frontend parser syntax.

## Acceptance criteria

- [ ] `classAttributeInferenceTemplateJS.ts` triage no longer reports `ClassDecl constructor FuncId 0 is out of range`.
- [ ] A focused regression covers a JS/noEmit class constructor with zero emitted functions.
- [ ] Existing class constructor lowering tests still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli class
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classAttributeInferenceTemplateJS.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classAttributeInferenceTemplateJS.ts --detail --no-dashboard-data
```

## Notes

Split from `issues/done/1173-implement-classAttributeInferenceTemplateJS.md`.
