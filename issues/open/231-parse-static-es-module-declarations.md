---
id: 231
title: "Parse static ES module declarations"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: [232]
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Replace issue-055 unsupported diagnostics for the initial static import/export forms with real frontend AST representation.

## Problem

The parser currently recognizes `import` and `export` only far enough to return issue-linked unsupported diagnostics. Downstream module work cannot begin until the frontend preserves module declarations with spans and specifiers.

## Desired final state

Basic static module declarations parse into AST nodes without lowering or executing modules yet.

## Scope

In scope:

- [ ] Add AST representation for side-effect import, default import, namespace import, named import, named export, and re-export declarations
- [ ] Preserve local names, imported names, exported names, module specifiers, and spans
- [ ] Keep unsupported diagnostics for dynamic `import()`, `export default`, and forms not implemented by this slice
- [ ] Add parser/frontend regression tests and fixtures for the supported declaration forms

Out of scope:

- [ ] Module graph construction
- [ ] Name resolution across files
- [ ] Lowering, backend emission, and runtime execution
- [ ] Dynamic import and CommonJS `require()`

## Affected paths

Expected:

- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser.rs`
- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/diagnostic.rs`
- `crates/frontend/src/`
- `fixtures/module-system/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`
- `docs/`

## Acceptance criteria

- [ ] Static import/export declarations listed in scope parse into explicit AST nodes
- [ ] Parser tests assert specifier/name/span preservation for each supported form
- [ ] Existing issue-055 unsupported diagnostic fixtures are either updated to parser-success fixtures or replaced by narrower unsupported cases
- [ ] Unsupported forms still produce issue-linked diagnostics
- [ ] No regression in existing frontend parser tests

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli static_named_import_reports_issue_055 static_named_export_reports_issue_055
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

This is parser/build-only work. Node differential evidence is not required until execution semantics are added.

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
