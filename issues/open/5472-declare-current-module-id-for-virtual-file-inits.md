---
id: 5472
title: "Declare current_module_id for virtual file inits"
type: bug
area: backend-wasm
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Emit the `$current_module_id` global whenever backend-wasm emits virtual
`@Filename` module initialization code that sets that global.

This is the current BackendIo blocker from
`noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts`.

## Problem

The representative file has a declaration-only `test.d.ts` section followed by
runtime code:

```ts
// @Filename: test.d.ts
declare class Something {
    private static someStaticVar;
    private someVar;
    private get getter();
    private set setter(v);
}

// @Filename: test.ts
var x = new Something();
```

Triage reaches WAT generation, but the final `wat2wasm` step fails because the
backend emits `(global.set $current_module_id ...)` without declaring
`$current_module_id`.

Problem: multi-file WAT init code can reference `$current_module_id` without
declaring the global, producing a BackendIo failure.

## Current failure

Reference triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts
```

Focused build reproduction:

```sh
/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm build reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts -o /tmp/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.wasm
```

Current result:

```text
error: [BackendIo] wat2wasm failed
/tmp/ts2wasm-2-0.wat:527:17: error: undefined global variable "$current_module_id"
    (global.set $current_module_id (i32.const 1))
                ^^^^^^^^^^^^^^^^^^
```

Focused coverage currently reports `build_pass=1` in batch mode, so the
direct build command above is the authoritative reproduction for the WAT
validation failure.

Compiler evidence:

```text
tokens: ok through declare class Something and var x = new Something()
ast: ok; runtime AST contains only Let x = New Something()
resolved: ok; resolved IR contains Let("x", New { class_name: "Something", args: [] })
wat: emits module init code with global.set $current_module_id
TypeScript oracle: ok, diagnostics=[]
```

## Desired final state

Any WAT that sets `$current_module_id` also declares the global, and the
representative no longer fails `wat2wasm` with an undefined global.

## Scope

In scope:

- [ ] Ensure backend-wasm declares `$current_module_id` when module init code or host runtime helpers reference it.
- [ ] Add a focused backend WAT validation test for virtual `@Filename` sections that trigger module init.
- [ ] Re-run `noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts` and record the next compiler result.

Out of scope:

- Full runtime semantics for external ambient classes.
- TypeScript diagnostics for implicit-any private members; the oracle reports no diagnostics for this representative.
- Broad module graph or CommonJS resolution changes.

## Affected paths

Expected:

- `crates/backend-wasm/src/emitter.rs`
- `crates/backend-wasm/src/lib.rs`
- focused backend tests or fixtures

Do not touch:

- frontend parser/resolver behavior unless the focused backend test proves the lowered module graph is malformed
- TypeScript semantic diagnostic machinery

## Acceptance criteria

- [ ] WAT containing `(global.set $current_module_id ...)` also contains a matching `(global $current_module_id (mut i32) ...)` declaration.
- [ ] A focused backend test compiles emitted WAT with `wat2wasm` for a virtual `@Filename` multi-file input.
- [ ] `/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm build reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts -o /tmp/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.wasm` no longer reports undefined `$current_module_id`.
- [ ] Any next blocker for the representative is recorded in this issue or split to a follow-up if outside this backend scope.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-backend-wasm
/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm build reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts -o /tmp/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.wasm
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.ts
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

Split from `issues/open/3533-implement-noImplicitAnyAndPrivateMembersWithoutTypeAnnotations.md`.

`crates/backend-wasm/src/emitter.rs` already emits `global.set
$current_module_id` in module init paths. The implementation should make the
global declaration contract match those use sites.

## Completion evidence

Fill when implemented.
