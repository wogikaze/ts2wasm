---
id: 5229
title: "Resolve imports between @Filename sections"
type: feature
area: compiler/module-graph
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Resolve local imports between virtual files declared by TypeScript reference
`// @Filename:` / `// @filename:` sections.

## Problem

`checkJsdocTypeTagOnExportAssignment2.ts` contains virtual files `a.ts`,
`b.js`, and `c.js`. The `c.js` section imports `./b`, but module graph
resolution looks on disk for `b.js` instead of resolving the virtual section.
`circularReferenceInImport.ts` has the same first blocker with virtual
`db.d.ts` and `app.ts` sections.

Problem: `import b from "./b"` in a `// @Filename: c.js` section reports `issue-232: missing local module ./b`.
Problem: `import * as Db from "./db"` in a `// @filename: app.ts` section reports `issue-232: missing local module ./db`.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts
```

Source shape:

```ts
// @Filename: b.js
/** @type {import("./a").Foo} */
export default { c: false };

// @Filename: c.js
import b from "./b";
b;
```

Compiler evidence:

```text
tokens: ok
ast: ok; ExportDefault, ImportDefault("./b"), Expr Ident("b")
module_graph: issue-232 missing local module ./b; tried on-disk b.ts/b.js/etc.
TypeScript oracle: TS2307 for ./b in this reference window
```

## Desired final state

For reference-style multi-section input, the module graph can resolve a local
specifier to another `@Filename` section in the same source file.

## Scope

In scope:

- [ ] Register `@Filename` / `@filename` section names as virtual module paths.
- [ ] Resolve `./b` from a `c.js` section to the sibling `b.js` section.
- [ ] Resolve `./db` from an `app.ts` section to the sibling `db.d.ts` section.
- [ ] Add one focused multi-section fixture using `export default` and a default import.

Out of scope:

- Package resolution and non-local module specifiers.
- Full TypeScript JSDoc type checking for `@type {import("./a").Foo}`.
- CommonJS `module.exports` lowering.
- All eight `checkJsdocTypeTagOnExportAssignment` variants.

## Affected paths

Expected:

- `crates/compiler/src/module_graph.rs`
- `crates/compiler/src/lib.rs`
- focused fixtures or compiler tests

Do not touch:

- `crates/backend-wasm/`
- unrelated frontend parser syntax

## Acceptance criteria

- [ ] `checkJsdocTypeTagOnExportAssignment2.ts` no longer reports `issue-232: missing local module ./b`.
- [ ] `circularReferenceInImport.ts` no longer reports `issue-232: missing local module ./db`.
- [ ] A focused compiler test proves `// @Filename: b.js` is resolved by `import "./b"` from another section.
- [ ] Existing missing real local module diagnostics still report issue-232.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler -E 'test(module) or test(filename)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkJsdocTypeTagOnExportAssignment2.ts --detail --no-dashboard-data
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

Split from generated bucket `issues/done/1138-implement-checkJsdocTypeTagOnExportAssignment.md`.
Also owns the matching first blocker folded from `issues/done/1162-implement-circularReferenceInImport.md`; see that closed bucket for full `./db` triage evidence.

## Completion evidence

Fill when implemented.
