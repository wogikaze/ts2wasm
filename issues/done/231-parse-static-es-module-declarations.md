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
completed: 2026-04-28
status: done
---

## Summary

Replace issue-055 unsupported diagnostics for the initial static import/export forms with real frontend AST representation.

## Problem

The parser currently recognizes `import` and `export` only far enough to return issue-linked unsupported diagnostics. Downstream module work cannot begin until the frontend preserves module declarations with spans and specifiers.

## Desired final state

Basic static module declarations parse into AST nodes without lowering or executing modules yet.

## Scope

In scope:

- [x] Add AST representation for side-effect import, default import, namespace import, named import, named export, and re-export declarations
- [x] Preserve local names, imported names, exported names, module specifiers, and spans
- [x] Keep unsupported diagnostics for dynamic `import()`, default function/class exports, variable exports, class exports, and forms not implemented by this slice
- [x] Add parser/frontend regression tests and fixtures for the supported declaration forms

Out of scope:

- Module graph construction
- Name resolution across files
- Lowering, backend emission, and runtime execution
- Dynamic import and CommonJS `require()`

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

- [x] Static import/export declarations listed in scope parse into explicit AST nodes
- [x] Parser tests assert specifier/name/span preservation for each supported form
- [x] Existing issue-055 unsupported diagnostic fixtures are either updated to parser-success fixtures or replaced by narrower unsupported cases
- [x] Unsupported forms still produce issue-linked diagnostics
- [x] No regression in existing frontend parser tests

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

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] dependent issues 232, 233, and 234 remain open for out-of-scope module graph, emission, and execution work

## Notes

This is parser/build-only work. Node differential evidence is not required until execution semantics are added.

## Progress evidence

2026-04-28 child worker `231-parse-static-esm-20260428T054045Z` completed a parser-only subset:

- Added frontend AST representation for side-effect imports, named imports, and local named exports with module specifier/name/span preservation.
- Converted those supported parser forms from issue-055 parser diagnostics into successful AST nodes.
- Kept namespace import, default import, star re-export, named re-export, and dynamic import issue-linked unsupported diagnostics.
- Added parser regression tests for supported forms and remaining unsupported forms.
- Added minimal downstream unsupported guards so full workspace compile remains green without implementing module graph, resolver, lowering, backend, or runtime semantics.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (40 tests)
cargo nextest run -p ts2wasm-cli static_named_import_reports_issue_055 static_named_export_reports_issue_055: PASS (2 tests)
cargo check --workspace: PASS
mise run check-issue-health: PASS
mise run check-agent-state: PASS
```

Remaining work before close:

- default import, namespace import, re-export declarations, export default, and declaration exports still need full parser AST coverage or narrower follow-up split.
- fixtures under `fixtures/module-system/` were not converted in this subset.

2026-04-28 child worker `231-static-esm-cont-20260428T055606Z` completed a parser-only continuation:

- Added frontend AST representation for standalone default imports with local binding, module specifier, and span preservation.
- Parsed `import value from "./module-source";` into `Stmt::ImportDefault`.
- Kept combined default imports with additional bindings, namespace imports, re-exports, dynamic import, and other unsupported module forms issue-linked.
- Added downstream unsupported guards so parsed default imports still stop before module graph, resolver, lowering, backend, or runtime semantics.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (41 tests)
cargo nextest run -p ts2wasm-cli static_default_import_reports_issue_055 static_named_import_reports_issue_055 static_named_export_reports_issue_055: PASS (3 tests)
cargo check --workspace: PASS
mise run check-issue-health: PASS
mise run check-agent-state: PASS
```

Remaining work before close:

- namespace import, re-export declarations, export default, combined default+named/default+namespace imports, and declaration exports still need parser AST coverage or narrower follow-up split.
- fixtures under `fixtures/module-system/` were not converted in this subset.

2026-04-28 child worker `231-namespace-import-20260428T060834Z` completed a parser-only namespace import continuation:

- Added frontend AST representation for standalone namespace imports with local namespace binding, module specifier, and span preservation.
- Parsed `import * as ns from "./module-source";` into `Stmt::ImportNamespace`.
- Kept combined default+namespace/default+named imports, re-exports, dynamic import, and other unsupported module forms issue-linked.
- Added downstream unsupported guards so parsed namespace imports still stop before module graph, resolver, lowering, backend, or runtime semantics.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (41 tests)
cargo nextest run -p ts2wasm-cli static_namespace_import_reports_issue_055 static_default_import_reports_issue_055 static_named_import_reports_issue_055 static_named_export_reports_issue_055: PASS (4 tests)
cargo check --workspace: PASS
```

Remaining work before close:

- re-export declarations, export default, combined default+named/default+namespace imports, and declaration exports still need parser AST coverage or narrower follow-up split.
- fixtures under `fixtures/module-system/` were not converted in this subset.

2026-04-28 child worker `231-re-export-parser-20260428T062802Z` completed a parser-only named re-export continuation:

- Added frontend AST representation for named re-exports with imported name, exported name, module specifier, and span preservation.
- Parsed `export { value as renamed } from "./module-source";` into `Stmt::ExportNamedFrom`.
- Kept star re-export, combined default imports, export default, declaration exports, dynamic import, and module execution semantics issue-linked or out of scope.
- Added downstream unsupported guards so parsed named re-exports still stop before module graph, resolver, lowering, backend, or runtime semantics.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (41 tests)
cargo nextest run -p ts2wasm-cli static_re_export_reports_issue_055 static_default_import_reports_issue_055 static_namespace_import_reports_issue_055 static_named_import_reports_issue_055 static_named_export_reports_issue_055 static_named_re_export_reports_issue_055: PASS (6 tests)
cargo check --workspace: PASS
mise run check-issue-health: PASS
mise run check-agent-state: PASS
cargo nextest run: PASS (342 tests, 4 skipped)
```

Remaining work before close:

- star re-export, export default, combined default+named/default+namespace imports, and declaration exports still need parser AST coverage or narrower follow-up split.
- broader fixtures under `fixtures/module-system/` still need conversion as forms become parsed.

2026-04-28 child worker `231-star-re-export-parser-20260428T065856Z` completed a parser-only star re-export continuation:

- Added frontend AST representation for star re-exports with `export *` span, module specifier, and declaration span preservation.
- Parsed `export * from "./module-source";` into `Stmt::ExportAllFrom`.
- Kept namespace re-export, export default, combined default imports, declaration exports, dynamic import, and module execution semantics issue-linked or out of scope.
- Added downstream unsupported guards so parsed star re-exports still stop before module graph, resolver, lowering, backend, or runtime semantics.
- Updated the CLI module guard fixture/test to prove parsed star re-exports still report issue-055 before module graph support.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (42 tests)
cargo nextest run -p ts2wasm-cli static_re_export_reports_issue_055 static_named_re_export_reports_issue_055 static_default_import_reports_issue_055 static_namespace_import_reports_issue_055 static_named_import_reports_issue_055 static_named_export_reports_issue_055: PASS (6 tests)
cargo check --workspace: PASS
cargo nextest run: PASS (344 tests, 4 skipped)
mise run check-issue-health: PASS
mise run check-agent-state: PASS
```

Remaining work before close:

- export default, combined default+named/default+namespace imports, namespace re-export, and declaration exports still need parser AST coverage or narrower follow-up split.
- broader fixtures under `fixtures/module-system/` still need conversion as forms become parsed.

2026-04-28 child worker `231-combined-import-parser-20260428T072707Z` completed a parser-only combined import continuation:

- Added frontend AST representation for combined default+named imports and combined default+namespace imports with default local, imported/local or namespace names, source specifier, and declaration spans preserved.
- Parsed `import defaultName, { value as renamed } from "./module-source";` into `Stmt::ImportDefaultNamed`.
- Parsed `import defaultName, * as ns from "./module-source";` into `Stmt::ImportDefaultNamespace`.
- Added downstream unsupported guards so parsed combined imports still stop with issue-055 before module graph, resolver, lowering, backend, or runtime semantics.
- Updated CLI module guard fixtures/tests to prove parsed combined imports still report issue-055 before module graph support, while preserving the standalone default-import guard fixture.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (43 tests)
cargo nextest run -p ts2wasm-cli static_default_import_reports_issue_055 static_combined_named_import_reports_issue_055 static_named_import_reports_issue_055 static_namespace_import_reports_issue_055 static_re_export_reports_issue_055 static_named_re_export_reports_issue_055 static_combined_namespace_import_reports_issue_055: PASS (7 tests after parent merge review fix)
cargo check --workspace: PASS
cargo nextest run: PASS (347 tests, 4 skipped)
mise run check-issue-health: PASS
mise run check-agent-state: PASS
```

Commit:

- `cc77a7a` (`issue-231: parse combined static imports`)

Remaining work before close:

- export default, namespace re-export, and declaration exports still need parser AST coverage or narrower follow-up split.
- broader fixtures under `fixtures/module-system/` still need conversion as forms become parsed.

2026-04-28 child worker `231-namespace-reexport-20260428T074900Z` completed a parser-only namespace re-export continuation:

- Added frontend AST representation for namespace re-exports with exported namespace name, exported name span, namespace specifier span, module specifier, and declaration span preservation.
- Parsed `export * as ns from "./module-source";` into `Stmt::ExportNamespaceFrom`.
- Added downstream unsupported guards so parsed namespace re-exports still stop with issue-055 before module graph, resolver, lowering, backend, or runtime semantics.
- Added a CLI module guard fixture/test to prove parsed namespace re-exports still report issue-055 before module graph support.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (43 tests)
cargo nextest run -p ts2wasm-cli static_namespace_re_export_reports_issue_055: PASS (1 test)
cargo check --workspace: PASS
mise run check-issue-health: PASS
mise run check-agent-state: PASS
cargo nextest run: PASS (350 tests, 4 skipped)
```

Remaining work before close:

- export default and declaration exports still need parser AST coverage or narrower follow-up split.
- broader fixtures under `fixtures/module-system/` still need conversion as forms become parsed.

2026-04-28 child worker `231-declaration-export-20260428T080100Z` completed a parser-only declaration export continuation:

- Added frontend AST representation for `export const value = 1;` as `Stmt::ExportDecl`, wrapping the existing declaration and preserving the exported local name span.
- Parsed the narrow `export const <ident> = <expr>;` form while keeping `export let`, `export var`, `export default`, and class declaration exports issue-linked or out of scope.
- Added downstream unsupported guards so parsed declaration exports still stop with issue-055 before module graph, resolver, lowering, backend, or runtime semantics.
- Added a CLI module guard fixture/test to prove parsed declaration exports still report issue-055 before module graph support.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (45 tests)
cargo nextest run -p ts2wasm-cli static_declaration_export_reports_issue_055: PASS (1 test)
cargo check --workspace: PASS
```

Remaining work before close:

- export default still needs parser AST coverage or narrower follow-up split.
- broader fixtures under `fixtures/module-system/` still need conversion as forms become parsed.

2026-04-28 child worker `231-export-default-20260428T082000Z` completed a parser-only export default continuation:

- Added frontend AST representation for `export default <expression>;` as `Stmt::ExportDefault`, preserving the default marker span, exported expression AST, and declaration span.
- Parsed the narrow expression default export form while keeping default function and default class exports issue-linked and out of scope.
- Added downstream unsupported guards so parsed default exports still stop with issue-055 before module graph, resolver, lowering, backend, or runtime semantics.
- Added a CLI module guard fixture/test to prove parsed default exports still report issue-055 before module loading support.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (47 tests)
cargo nextest run -p ts2wasm-cli static_default_export_reports_issue_055: PASS (1 test)
cargo check --workspace: PASS
mise run check-issue-health: PASS
mise run check-agent-state: PASS
cargo nextest run: PASS (356 tests, 4 skipped)
```

Commit:

- `bb6e2b3` (`issue-231: parse export default expression`)

Remaining work before close:

- broader fixtures under `fixtures/module-system/` still need conversion as forms become parsed.

2026-04-28 child worker `231-close-audit-20260428T083200Z` performed a close-readiness audit and did not close the issue.

Verified acceptance coverage:

- Current frontend parser tests cover explicit AST nodes and span/specifier/name preservation for side-effect imports, named imports, default imports, combined default imports, namespace imports, named exports, named re-exports, star re-exports, namespace re-exports, `export const`, and expression `export default`.
- Current CLI module guard tests cover all `static_*_reports_issue_055` fixtures and prove parsed static module declarations still stop with issue-055 before module graph/resolution/lowering/runtime support.
- `cargo fmt --all --check`: PASS
- `cargo nextest run -p ts2wasm-frontend`: PASS (47 tests)
- `cargo nextest run -p ts2wasm-cli static_named_import_reports_issue_055 static_side_effect_import_reports_issue_055 static_namespace_import_reports_issue_055 static_default_import_reports_issue_055 static_combined_named_import_reports_issue_055 static_combined_namespace_import_reports_issue_055 static_named_export_reports_issue_055 static_re_export_reports_issue_055 static_named_re_export_reports_issue_055 static_namespace_re_export_reports_issue_055 static_declaration_export_reports_issue_055 static_default_export_reports_issue_055`: PASS (12 tests)

Blocker:

- Acceptance criterion "Unsupported forms still produce issue-linked diagnostics" is not fully met: `export class C {}` currently builds successfully instead of producing an issue-055 unsupported module diagnostic, because the parser consumes `export` and returns a plain `Stmt::ClassDecl`. Control probes confirmed `export function f() {}` and `export var value = 1;` do produce issue-055 diagnostics. Leave 231 open until `export class` is either represented as an export declaration AST node or rejected with an issue-linked unsupported diagnostic.

2026-04-28 child worker `231-export-class-guard-20260428T083349Z` resolved the close-readiness blocker as a targeted unsupported guard:

- Changed `export class C {}` parsing to produce the existing issue-055 unsupported module diagnostic (`unsupported class export`) instead of returning a plain `Stmt::ClassDecl`.
- Added a frontend parser regression for the issue-linked `export class` diagnostic.
- Added `fixtures/module-system/static-class-export-unsupported.ts` and CLI regression coverage proving the build no longer succeeds silently.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS
cargo nextest run -p ts2wasm-cli static_class_export_reports_issue_055: PASS
mise run check-issue-health: PASS
mise run check-agent-state: PASS
```

Remaining work before close:

- Full issue close still requires parent/orchestrator review of all acceptance criteria and issue lifecycle movement.

## Completion evidence

Closed by final close audit `231-final-close-audit-20260428T084159Z` after the export-class unsupported guard merge.

Commits:

- `a42f5a0` (`Merge issue 231 export class guard`)

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-28

command: cargo nextest run -p ts2wasm-frontend
result: PASS (48 tests)
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli static_named_import_reports_issue_055 static_side_effect_import_reports_issue_055 static_namespace_import_reports_issue_055 static_default_import_reports_issue_055 static_combined_named_import_reports_issue_055 static_combined_namespace_import_reports_issue_055 static_named_export_reports_issue_055 static_re_export_reports_issue_055 static_named_re_export_reports_issue_055 static_namespace_re_export_reports_issue_055 static_declaration_export_reports_issue_055 static_default_export_reports_issue_055 static_class_export_reports_issue_055
result: PASS (13 tests)
date: 2026-04-28

command: mise run check-issue-health
result: PASS
date: 2026-04-28

command: mise run check-agent-state
result: PASS
date: 2026-04-28

command: mise run update-issue-index
result: PASS
date: 2026-04-28

command: mise run update-issue-index -- --check
result: PASS
date: 2026-04-28

command: mise run check-issue-index
result: PASS
date: 2026-04-28

command: cargo nextest run
result: PASS (358 tests, 4 skipped)
date: 2026-04-28
```

Remaining risks:

- Module graph construction, name resolution across files, lowering, backend emission, and runtime execution remain intentionally out of scope and tracked by dependent issues 232, 233, and 234.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

