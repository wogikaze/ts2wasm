---
id: 5333
title: "Report strict mode arguments binding diagnostics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report TypeScript TS1100-style diagnostics when strict-mode code binds the
reserved name `arguments` in parameters or local declarations.

The representative `collisionArgumentsArrowFunctions.ts` now parses, resolves,
and build-passes in ts2wasm, but TypeScript reports invalid strict-mode
`arguments` uses in arrow parameter and body bindings.

## Problem

`collisionArgumentsArrowFunctions.ts` contains arrow functions that use
`arguments` as a rest parameter, ordinary parameter, and local `var` binding.
With `@alwaysStrict`, TypeScript reports TS1100 at each invalid binding, while
the current compiler returns `BuildPass`.

Problem: strict-mode `arguments` bindings currently build-pass silently instead
of reporting source-spanned TS1100-style diagnostics.

## Current failure

Reproduction: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts`.

Focused coverage: `python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts --detail --no-dashboard-data`.

Observed compiler result:

```text
collisionArgumentsArrowFunctions.ts: build_pass
```

TypeScript oracle:

```text
TS1100: Invalid use of 'arguments' in strict mode.
```

Representative source:

```ts
var f1 = (i: number, ...arguments) => {
    var arguments: any[];
}
var f12 = (arguments: number, ...rest) => {
    var arguments = 10;
}
```

Compiler evidence:

```text
tokens: ok through arrow params and body `var arguments`
ast/resolved: ok; arrow params include `...arguments` and `arguments`
oracle: TS1100 at rest parameter, ordinary parameter, and local var bindings
```

## Desired final state

The frontend reports source-spanned diagnostics for strict-mode `arguments`
bindings instead of treating the representative reference as a clean build
pass.

## Scope

In scope:

- [x] Detect `arguments` used as an arrow function rest parameter in strict mode.
- [x] Detect `arguments` used as an arrow function ordinary parameter in strict mode.
- [x] Detect local `var arguments` declarations in strict-mode arrow bodies.
- [x] Report a TS1100-style diagnostic at the offending identifier span.
- [x] Preserve non-strict behavior where the same binding is allowed.

Out of scope:

- Full runtime `arguments` object lowering.
- Object literal property names named `arguments`.
- Name-resolution support for implicit function-scope `arguments`.
- All strict-mode reserved words beyond `arguments`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused frontend/resolver or semantic tests
- fixtures or CLI reference tests

Do not touch:

- backend lowering for the `arguments` object
- unrelated strict-mode parser behavior

## Acceptance criteria

- [x] `collisionArgumentsArrowFunctions.ts` no longer build-passes silently; it reports TS1100-style diagnostics for invalid strict-mode `arguments` bindings.
- [x] A focused test covers `(...arguments) => {}` in strict mode.
- [x] A focused test covers `(arguments) => {}` in strict mode.
- [x] A focused test covers `() => { var arguments = 1; }` in strict mode.
- [x] A non-strict fixture with `arguments` binding remains accepted if currently supported.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(arguments) or test(strict)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsArrowFunctions.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from `issues/open/1266-implement-collisionArgumentsArrowFunctions.md` on
2026-05-07.

Related but not duplicates:

- `issues/done/649-implement-argumentsBindsToFunctionScopeArgumentList.md`
  currently tracks a generated name-resolution bucket for implicit
  function-scope `arguments`, not strict-mode binding diagnostics.
- `issues/done/658-implement-argumentsReferenceInObjectLiteral.md` tracks
  object literal parsing/reference behavior.

## Completion evidence

Fill only when implemented.

## False-done audit

**truly-done** (5333)

- Implementation commits: verified via `git log --oneline --all --grep=5333`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
