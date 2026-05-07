---
id: 711
title: "Report TS1108 for top-level return statements"
type: feature
area: compiler/diagnostics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Map top-level `return` validation failures to a TypeScript-style TS1108 diagnostic instead of leaving them as generic unsupported `InvalidTopLevelReturn` blockers.

## Problem

The compiler already parses top-level `return` statements and rejects them during validation with `InvalidTopLevelReturn`. TypeScript reports the same source shape as diagnostic TS1108:

```text
A 'return' statement can only be used within a function body.
```

Problem: top-level `return` currently reports unsupported `InvalidTopLevelReturn` instead of a TS1108-style diagnostic.

Because the current diagnostic remains a project-internal unsupported code, reference coverage marks these cases as unsupported instead of expected TypeScript diagnostics.

Representative coverage for `asiReturn.ts`:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/asiReturn.ts --detail --no-dashboard-data
```

Current result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=InvalidTopLevelReturn:1
unsupported_features=top-level-return:1
```

Representative coverage for `multiLinePropertyAccessAndArrowFunctionIndent1.ts`:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiLinePropertyAccessAndArrowFunctionIndent1.ts --detail --no-dashboard-data
```

Current result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=InvalidTopLevelReturn:1
unsupported_features=top-level-return:1
```

## Source Context

Minimal ASI case:

```ts
// @target: es2015
// This should be an error for using a return outside a function, but ASI should work properly
return
```

Multi-line property access and arrow function case:

```ts
// @target: es2015
// @strict: false
return this.edit(role)
    .then((role: Role) =>
        this.roleService.add(role)
            .then((data: ng.IHttpPromiseCallbackArg<Role>) => data.data));
```

Focused triage shows the parser produces a top-level `Return` AST in both cases. The first case has `Undefined` as the return expression because ASI applies; the second keeps the full chained call expression. Validation then emits `InvalidTopLevelReturn`.

TypeScript oracle evidence:

```text
asiReturn.ts: TS1108 at the top-level return keyword
multiLinePropertyAccessAndArrowFunctionIndent1.ts: TS1108 at the top-level return keyword
```

## Desired final state

Top-level `return` validation errors are surfaced as a TypeScript-style TS1108 diagnostic with the source span on the `return` keyword. The representative reference cases should no longer be counted as unsupported `InvalidTopLevelReturn` blockers.

## Scope

In scope:

- [ ] Map `InvalidTopLevelReturn` validation failures to TS1108-compatible diagnostic output.
- [ ] Preserve the diagnostic span on the `return` keyword, not the full return expression.
- [ ] Cover bare ASI `return`, multi-line `return <expression>`, and valid function-body returns.

Out of scope:

- Implementing top-level return as executable JavaScript.
- Changing parser ASI behavior beyond preserving the existing top-level `Return` AST.
- Diagnostics for top-level `break`, `continue`, `await`, or `yield`.
- Runtime, WASM backend, or module-system changes.

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- compiler or CLI diagnostic tests that assert diagnostic code/span

Do not touch:

- frontend parser behavior unless a span preservation bug is found
- backend/runtime lowering

## Acceptance criteria

- [ ] `reference/typescript/tests/cases/compiler/asiReturn.ts` no longer reports unsupported `InvalidTopLevelReturn`; it reports a TS1108-style diagnostic.
- [ ] `reference/typescript/tests/cases/compiler/multiLinePropertyAccessAndArrowFunctionIndent1.ts` no longer reports unsupported `InvalidTopLevelReturn`; it reports a TS1108-style diagnostic.
- [ ] Focused regression coverage asserts the `return` keyword span and keeps valid in-function returns passing.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/asiReturn.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiLinePropertyAccessAndArrowFunctionIndent1.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/asiReturn.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiLinePropertyAccessAndArrowFunctionIndent1.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

This issue now owns the shared top-level return diagnostic family. It absorbs the `asiReturn.ts` generated bucket and the duplicate evidence from #3405. Done duplicate #946 already points at this issue.

## Completion evidence

Fill only when implemented.
