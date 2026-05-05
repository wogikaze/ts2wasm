---
id: 5155
title: "Fix exception_pending runtime link for top-level statement checks"
type: bug
area: backend-wasm
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

The backend emits a top-level pending-exception guard after each lowered top-level statement, but the runtime link plan does not declare `$exception_pending` for statement-only programs that otherwise do not select exception runtime globals.

This causes valid WAT generation to fail during `wat2wasm` for `baseTypeAfterDerivedType.ts` once parsing/resolution succeed.

## Problem

Problem: `reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts` reaches backend emission, then `wat2wasm` fails because `$exception_pending` is referenced but not declared.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts
```

Direct build reproduction:

```sh
cargo run -q -p ts2wasm-cli -- build reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts -o /tmp/ts2wasm-1038-baseTypeAfterDerivedType.wasm
```

Current stderr excerpt:

```text
error: [BackendIo] wat2wasm failed
/tmp/ts2wasm-2-0.wat:753:21: error: undefined global variable "$exception_pending"
    (if (global.get $exception_pending)
                    ^^^^^^^^^^^^^^^^^^
```

Representative source:

```ts
interface Derived extends Base {
    method(...args: any[]): void;
}

interface Base {
    method(...args: any[]): void;
}

class Derived2 implements Base2 {
    method(...args: any[]) {}
}
```

Current compiler evidence:

- Tokens, AST, and resolved dumps succeed.
- Interfaces are erased.
- Resolved IR contains `ClassDecl Derived2` with method `method` and one rest parameter `args`.
- TypeScript oracle succeeds with no diagnostics; parameter hints report `args: any[]`.
- WAT dump succeeds, but final `wat2wasm` validation fails on missing `$exception_pending`.

## Desired final state

Any emitted WAT that references `$exception_pending` declares the corresponding runtime global through the runtime link plan. `baseTypeAfterDerivedType.ts` should no longer fail with an undefined `$exception_pending` global.

## Scope

In scope:

- [ ] Make top-level statement exception guards select the exception runtime global contract, or otherwise avoid emitting the guard when the global is not present.
- [ ] Add a backend/runtime-link regression that catches `$exception_pending` references without declarations.
- [ ] Add or update a reference/fixture regression for `baseTypeAfterDerivedType.ts` or an equivalent class method/rest-parameter top-level statement program.

Out of scope:

- TypeScript interface inheritance type-checking semantics.
- Broad rest-parameter runtime semantics beyond preserving the existing parsed/resolved shape.
- Parser changes for `implements`, interface members, or rest parameters unless a regression proves the backend fix is insufficient.

## Affected paths

Expected:

- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/stmt_emit.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/lib.rs`
- `fixtures/` or backend wasm tests

Do not touch:

- `crates/frontend/src/parser/` unless the backend regression still fails after the runtime-link contract is fixed.

## Acceptance criteria

- [ ] `cargo run -q -p ts2wasm-cli -- build reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts -o /tmp/ts2wasm-1038-baseTypeAfterDerivedType.wasm` no longer fails with `undefined global variable "$exception_pending"`.
- [ ] A regression asserts that emitted WAT cannot reference `$exception_pending` without declaring it.
- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts` no longer reports the same `BackendIo`/`wat2wasm failed` diagnostic.
- [ ] Runtime-link structure tests cover the selected exception global path.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-backend-wasm
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts
cargo run -q -p ts2wasm-cli -- build reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts -o /tmp/ts2wasm-1038-baseTypeAfterDerivedType.wasm
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

The observed WAT guard is emitted from top-level statement handling after each statement. This issue is about keeping runtime-link global selection synchronized with emitted WAT, not about broad exception semantics.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
