---
id: 5447
title: "Support instanceof callable prototype RHS"
type: feature
area: ir/runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Support `instanceof` right-hand sides that are callable/prototype-bearing
values, such as a function parameter typed `{ (): void; prototype: A; }`.

Split from generated bucket `issues/done/3442-implement-narrowByInstanceof.md`.

## Problem

`narrowByInstanceof.ts` parses successfully, but lowering reports issue-207 for
`x instanceof A`, where `A` is a function parameter with a callable/prototype
type alias rather than a class declaration.

Problem: `instanceof` RHS resolution only accepts the currently supported class
constructor shape and rejects callable values that TypeScript treats as
constructor/prototype-compatible.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByInstanceof.ts
```

Current diagnostic:

```text
UnsupportedSyntax: issue-207: instanceof right-hand side must be a supported class constructor `A`
```

Focused coverage: `executed=1`, `unsupported=1`,
`unsupported_diagcodes=UnsupportedSyntax:1`.

Source context:

```ts
type AA = {
    (): void;
    prototype: A;
}

function foo(x: A | B | C, A: AA, B: BB, AB: AA | BB) {
    if (x instanceof A) {
        x;
    }
}
```

Compiler evidence: tokens and AST are ok; `x instanceof A`, `B`, and `AB` are
parsed as `InstanceOf`; lower_program reports issue-207 for RHS `A`.
TypeScript accepts the file with no diagnostics.

## Desired final state

The compiler recognizes supported callable/prototype-bearing RHS values for
`instanceof`, or emits a narrower source-backed diagnostic that distinguishes
this shape from unsupported class constructor values.

## Scope

In scope:

- [ ] Classify `instanceof` RHS identifiers that resolve to callable/prototype
      typed values.
- [ ] Support or precisely diagnose `x instanceof A`, `x instanceof B`, and
      `x instanceof AB` from the representative path.
- [ ] Preserve existing direct class constructor `instanceof` behavior.
- [ ] Re-triage `narrowByInstanceof.ts` and record the next blocker if it
      advances to class-value or narrowing semantics.

Out of scope:

- Full TypeScript narrowing semantics for `instanceof`.
- Arbitrary runtime function objects without prototype evidence.
- First-class class constructor values, tracked by issue 5192.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/` if runtime constructor/prototype support is needed
- focused fixtures or CLI tests

Do not touch: parser syntax unless fresh implementation evidence shows a parser
regression.

## Acceptance criteria

- [ ] `narrowByInstanceof.ts` no longer reports the generic issue-207
      diagnostic for RHS `A`.
- [ ] A focused fixture covers `x instanceof A` where `A` is a callable value
      with a `prototype` property.
- [ ] Existing direct class `instanceof` fixtures still pass.
- [ ] Unsupported RHS values keep a source-backed diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(instanceof) or test(class) or test(call)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByInstanceof.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByInstanceof.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Related completed issue 207 implemented the prototype-chain operator for
supported constructor shapes. This issue owns the unresolved callable/prototype
RHS classification reached by TypeScript narrowing tests.

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
