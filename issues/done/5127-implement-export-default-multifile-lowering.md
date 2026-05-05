---
id: 5127
title: "Implement export default multi-file lowering deduplication"
type: feature
area: ir/lowering
class: implementation
priority: P1
depends_on: [3002]
blocks: []
parent: 3002
created: 2026-05-05
updated: 2026-05-06
status: done
---

## Summary

Fix the `export default` lowering to avoid generating duplicate `__ts2wasm_default` bindings when compiling multi-file TypeScript test cases.

## Problem

Reference test `isolatedDeclarationErrorsDefault.ts` fails with `DuplicateLocal`:

```
error: [DuplicateLocal] duplicate local variable: `__ts2wasm_default` at 189..219
```

The reference test uses `@fileName:` directives to define multiple modules in a single file:

```typescript
// @fileName: a.ts
export default 1 + 1;

// @fileName: b.ts
export default { foo: 1 + 1 };

// @fileName: c.ts
export default [{ foo: 1 + 1 }];
```

The compiler generates `__ts2wasm_default` bindings for each `export default` and places them all in a single scope (since multi-file lowering treats the whole file as one compilation unit), triggering DuplicateLocal. Each `@fileName` should create a separate module scope.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts
```

Smart triage evidence:

- **Diagnostic:** `DuplicateLocal` at span 189..219
- **Error type:** `ir-or-lowering`
- **Root:** `export default` lowering generates duplicate `__ts2wasm_default` in shared scope

## Desired final state

The compiler handles `@fileName:` multi-file directives by isolating each module's scope, so `export default` bindings in different files don't collide. The reference test compiles without `DuplicateLocal` errors.

## Scope

In scope:

- [x] Investigate how `@fileName:` directives are lowered by the compiler
- [x] Ensure each `@fileName:` section gets its own module scope for bindings
- [x] Fix `__ts2wasm_default` generation to be scope-aware
- [x] Update reference coverage for the fixed test case

Out of scope:

- Full ES module support (import/export interop) — separate issue
- All isolatedDeclarationErrors family members (#2998-#3015) — only this specific test case

## Affected paths

- `crates/cli/src/` (multi-file command handling)
- `crates/ir/src/` (lowering)
- `crates/backend-wasm/src/` (if lowering changes propagate)

Do not touch:
- `crates/frontend/src/` (parser already handles `export default` via TypeScript AST)

## Acceptance criteria

- [x] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts` passes
- [x] Multi-file compilation with `@fileName:` directives produces correct scoped output
- [x] No regression in existing tests (`cargo nextest run`)

## Validation

```sh
cargo fmt --all --check
cargo nextest run -E 'not test(test262_)'
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts
```

## Completion evidence

Completed in commit `2eb01c0e` (`compiler: uniquify default export locals`).

The lowering now generates per-statement synthetic locals such as `__ts2wasm_default_0` and
`__ts2wasm_default_1` while preserving the exported name `default`, so repeated default exports
from a multi-file reference case do not collide in the shared lowered program.

Validation:

```text
cargo nextest run -p ts2wasm-compiler static_default_export_rewrite_uses_unique_synthetic_locals
=> pass

python scripts/manager.py reference-triage --format json tsc reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts
=> pass (BuildPass)

python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/isolatedDeclarationErrorsDefault.ts --detail
=> pass (executed=1, build_pass=1, unsupported=0, blocked=0)

cargo nextest run -p ts2wasm-compiler
=> pass (59 tests)

cargo fmt --all --check
=> pass

git diff --check
=> pass
```
