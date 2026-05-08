---
id: 5417
title: "Parse default keyword in named import specifiers"
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

Accept `default` as the imported name in named import specifiers, starting with
`import { default as b } from "./mod.cjs";`.

## Problem

`moduleNodeDefaultImports.ts` tokenizes `Default` correctly but the named import
specifier parser expects a normal identifier and stops before later module
resolution diagnostics are reachable.

Problem: the parser should accept contextual `default` as the imported binding
name in named import specifiers such as `import { default as b } from "./mod.cjs";`.

Current diagnostic:

```text
UnsupportedSyntax: expected identifier, got Some(SpannedToken { kind: Default, span: Span { start: 196, end: 203 } }) at 204..206
```

## Current failure

Reproduction: `env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts`

Coverage: `env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts --detail --no-dashboard-data`

Observed:

```text
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Source context:

```ts
declare function fun(): void;
export default fun;
import a from "./mod.cjs";
import { default as b } from "./mod.cjs";
import c, { default as d } from "./mod.cjs";
```

Evidence:

```text
tokens: ok; `Default` tokens are preserved in named import specifier lists.
ast/resolved: fail while parsing `default as b`.
visible symbols: []
TypeScript AST: ImportDeclaration exists for `import { default as b } ...`.
```

## Desired final state

The parser represents `default as b` inside named import specifiers, preserving
imported name, local alias, source specifier, and spans.

## Scope

In scope:

- [ ] Allow `import { default as b } from "./m";` in the named import
      specifier parser.
- [ ] Add focused frontend parser/module tests for the contextual `default`
      imported name.
- [ ] Re-run the representative triage and record the next diagnostic.

Out of scope:

- Re-export specifiers unless they share the same helper.
- Node16/NodeNext resolution and `.cjs` / `.mjs` virtual filename resolution.
- TS2528 multiple default export diagnostics.
- Default import/export runtime semantics.

## Affected paths

Expected: `crates/frontend/src/parser/statements_general.rs` and focused frontend parser/module tests.

Do not touch: backend/runtime code, package resolution, or Node module resolution.

## Acceptance criteria

- [ ] `import { default as b } from "./mod.cjs";` no longer reports `expected identifier` at `default`.
- [ ] The parser preserves imported `default`, local alias `b`, and spans.
- [ ] `moduleNodeDefaultImports.ts` advances past the current `default as b` parser blocker or records the next narrower diagnostic.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend import
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleNodeDefaultImports.ts --detail --no-dashboard-data
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

Split from `issues/open/3353-implement-moduleNodeDefaultImports.md`.

Related but not duplicates:

- `issues/open/5367-support-named-default-class-export-declarations.md` covers `export default class Name {}`.
- `issues/open/5403-support-type-only-default-exports-of-local-interfaces.md` covers resolving `export default InterfaceName;`.

## Completion evidence

Fill when implemented.
