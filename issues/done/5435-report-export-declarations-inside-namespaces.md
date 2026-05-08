---
id: 5435
title: "Report export declarations inside namespaces"
type: bug
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TypeScript-style diagnostic for `export { ... }` declarations inside namespace bodies instead of erasing the namespace as a clean build-pass.

## Problem

`multipleExports.ts` contains `export {x};` inside `export namespace M`. ts2wasm erases the namespace and build-passes with only the top-level `const x`, while TypeScript reports TS1194 and TS2484.

Problem: namespace-body export declarations are erased before diagnostics, causing a false build-pass.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleExports.ts
```

Observed:

```text
BuildPass: ts2wasm build succeeded
TypeScript oracle: TS1194 Export declarations are not permitted in a namespace; TS2484 conflict for x
```

## Source Context

```ts
export namespace M {
    export var v = 0;
    export let x;
}
const x = 0;
export namespace M {
    v;
    export {x};
}
```

Tokens include the nested `Export LeftBrace Ident("x") RightBrace` sequence, but the retained AST only contains the top-level `const x`.

## Desired final state

The frontend reports a source-spanned diagnostic for `export {x};` inside a namespace body. The representative should no longer build-pass silently.

## Scope

In scope:

- [ ] Detect namespace-body `export { ... }` declarations.
- [ ] Report a diagnostic at the nested `export {x}` span.
- [ ] Add focused coverage for `namespace M { export {x}; }`.

Out of scope:

- Top-level `export namespace` syntax recognition; tracked by #5352.
- Namespace value binding or runtime lowering.
- General ES module export declarations outside namespaces.
- Full duplicate exported-name checking beyond the representative `x` conflict.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/`
- focused frontend or CLI diagnostic tests

Do not touch:

- backend emit
- package/module resolution
- runtime ABI

## Acceptance criteria

- [ ] `multipleExports.ts` no longer build-passes silently; it reports a diagnostic for `export {x};` inside namespace `M`.
- [ ] A focused test covers a namespace body containing `export {x};`.
- [ ] Valid namespace member exports such as `export var v = 0;` remain accepted or erased according to existing namespace policy.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(export)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleExports.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleExports.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Notes

Split from #3413 on 2026-05-08. This issue is narrower than #5352: the representative already tokenizes and erases `export namespace`; the missing behavior is the nested export declaration diagnostic.
