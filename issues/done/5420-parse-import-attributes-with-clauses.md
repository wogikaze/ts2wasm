---
id: 5420
title: "Parse import attributes with clauses"
type: feature
area: frontend/module-syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Parse static import attributes such as
`import data from "./data.json" with { type: "json" };` without treating the
`with` keyword as an unexpected statement continuation.

## Problem

`modulePreserve5.ts` tokenizes the static import and following `with` clause,
but the import declaration parser expects the statement to end after the module
specifier and reports `expected Semicolon`.

Problem: import attribute `with { type: "json" }` clauses are not accepted or
diagnosed at the module import attribute boundary.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve5.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Ident("with")) at 196..200
```

Source context:

```ts
// @Filename: main.ts
import data1 from "./data.json" with { type: "json" };
const data2 = await import("./data.json", { with: { type: "json" } });
```

Compiler evidence:

```text
tokens: ok; static import tokens include Ident("with"), LeftBrace, Ident("type"), Colon, String("json"), RightBrace.
visible symbols: []
ast/resolved: fail before representing the static import declaration attributes.
```

TypeScript oracle:

```text
Top level includes ImportDeclaration `import data1 from "./data.json" with { type: "json" };`
and FirstStatement for `const data2 = await import("./data.json", { with: { type: "json" } });`.
```

## Desired final state

The frontend parses or explicitly records static import attributes on import
declarations so the representative advances beyond the `with` token.

## Scope

In scope:

- [ ] Accept `with { type: "json" }` after a string module specifier in static import declarations.
- [ ] Preserve the attribute clause span or emit a source-spanned unsupported import-attributes diagnostic.
- [ ] Add one focused parser/module syntax test for a static import attribute clause.

Out of scope:

- Dynamic import second-argument attributes.
- JSON module resolution or `resolveJsonModule` behavior.
- Runtime loading of JSON modules.
- Full import-attribute object expression semantics beyond the static `{ type: "json" }` shape.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs` if import declarations need an attribute field
- focused frontend module syntax tests

Do not touch:

- runtime/backend JSON loading
- package/module resolution
- top-level await lowering

## Acceptance criteria

- [ ] `import data1 from "./data.json" with { type: "json" };` no longer reports `expected Semicolon` at `with`.
- [ ] A focused test preserves or diagnoses the import attribute clause with the correct source span.
- [ ] Plain static imports without attributes continue to parse through the existing path.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend import
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve5.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserve5.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
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

Split from `issues/done/3358-implement-modulePreserve.md`.

This issue intentionally targets static import declarations first. After it
lands, `modulePreserve5.ts` may expose the dynamic import second-argument
attribute object, top-level await, or JSON module resolution as separate
blockers.

## Completion evidence

Fill when implemented.
