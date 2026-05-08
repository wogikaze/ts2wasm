---
id: 5400
title: "Parse exported import-equals declarations"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Parse TypeScript `export import name = qualified.name;` alias declarations
instead of reporting the generic issue-055 static export boundary.

## Problem

`aliasesInSystemModule2.ts` currently parses a named import and a plain
import-equals alias, then stops when the same import-equals form is exported:

```ts
import {alias} from "foo";
import cls = alias.Class;
export import cls2 = alias.Class;
```

Problem: exported import-equals declarations stop at generic issue-055 static export before the parser can preserve the alias declaration shape.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts
```

Observed result:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 118..124
```

Source context:

```text
5 | import {alias} from "foo";
6 | import cls = alias.Class;
7 | export import cls2 = alias.Class;
8 |
9 | let x = new alias.Class();
```

Compiler evidence:

```text
tokens: ok through Export, Import, Ident("cls2"), Equal, Ident("alias"), Dot, Ident("Class"), Semicolon
ast: fails at the `export` keyword before creating an exported ImportEqualsDeclaration
TypeScript oracle: topLevel includes ImportEqualsDeclaration "export import cls2 = alias.Class;"
```

## Desired final state

The frontend represents exported import-equals declarations with enough
structure and span information to advance `aliasesInSystemModule2.ts` past the
current generic static export boundary to the next narrower module, alias
resolution, or namespace diagnostic.

## Scope

In scope:

- [ ] Parse `export import name = qualified.name;` as an exported import-equals declaration.
- [ ] Preserve the exported flag, alias name span, and qualified target span.
- [ ] Cover the top-level form from `aliasesInSystemModule2.ts`.
- [ ] Re-triage `aliasesInSystemModule2.ts` and record the next diagnostic.

Out of scope:

- Resolving the imported module `"foo"`.
- Resolving `alias.Class` or constructor calls through the alias.
- CommonJS `import name = require("...")` module loading.
- Full SystemJS or isolatedModules emit behavior.

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/`
- focused parser or CLI AST tests

Do not touch:

- backend/runtime emit unless parsing cannot surface a controlled unsupported diagnostic
- package or node module resolution

## Acceptance criteria

- [ ] `export import cls2 = alias.Class;` no longer reports `issue-055: unsupported static export` as its first blocker.
- [ ] A focused parser or CLI AST test covers exported import-equals with a qualified target.
- [ ] `env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts` advances past the current `118..124` static export boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend import
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasesInSystemModule2.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter aliasesInSystemModule --detail --no-dashboard-data
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

Split from `issues/open/595-implement-aliasesInSystemModule.md`.

Related but not duplicates:

- `issues/open/5295-resolve-import-equals-require-to-virtual-node-modules-class-export.md` covers `require(...)` resolution through virtual node_modules.
- `issues/open/5398-resolve-namespace-import-equals-alias-value-access.md` covers value access after import-equals aliases are already parsed.
- `issues/open/5262-resolve-import-equals-aliases-in-class-implements-clauses.md` covers class heritage resolution for parsed import-equals aliases.

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
