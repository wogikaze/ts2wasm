# Cycle Report: 231 Re-export Parser

Run id: `231-re-export-parser-20260428T062802Z`
Issue: `issues/open/231-parse-static-es-module-declarations.md`
Branch: `agent/231-re-export-parser-20260428T062802Z`
Commit: `c298f6a`
Status: PROGRESS

## Scope

Implemented the parser-only named re-export slice:

- `export { value as renamed } from "./module-source";` parses into frontend AST.
- Imported name, exported name, module specifier, and spans are preserved.
- Star re-export remains an issue-055 unsupported parser diagnostic.
- Module graph, resolver semantics, lowering, backend emission, and runtime execution remain out of scope.

## Changes

- Added `ReExportNamedSpecifier` and `Stmt::ExportNamedFrom`.
- Parsed named re-exports with source specifiers in `crates/frontend/src/parser.rs`.
- Added frontend parser regression coverage for name/source/span preservation.
- Added unparse support for `ExportNamedFrom` in dump pseudo-source.
- Added compiler/IR downstream unsupported guards so parsed named re-exports stop with issue-055.
- Added `fixtures/module-system/static-named-re-export-unsupported.ts` and a CLI guard test.

## Validation

All commands passed on 2026-04-28 UTC:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (41 tests)
cargo nextest run -p ts2wasm-cli static_re_export_reports_issue_055 static_default_import_reports_issue_055 static_namespace_import_reports_issue_055 static_named_import_reports_issue_055 static_named_export_reports_issue_055 static_named_re_export_reports_issue_055: PASS (6 tests)
cargo check --workspace: PASS
scripts/manager check-issue-health: PASS
scripts/manager check-agent-state: PASS
cargo nextest run: PASS (342 tests, 4 skipped)
```

## Acceptance Evidence

- Supported form: `parser::tests::parses_named_re_export_with_specifier_and_source_spans` verifies `ExportNamedFrom` plus imported/exported names, module specifier, and spans.
- Unsupported form retained: `parser::tests::rejects_re_export_with_issue_linked_diagnostic` keeps `export * from "./module-source";` issue-linked.
- Downstream guard: `static_named_re_export_reports_issue_055` confirms parsed named re-export still reports `issue-055: unsupported named re-export` before module resolution/loading.

## Remaining Work

Issue 231 remains open. Remaining parser/frontend forms include at least:

- star re-export AST coverage
- export default
- combined default+named/default+namespace imports
- declaration exports
- broader fixture conversion under `fixtures/module-system/`

## Close Decision

Not closing issue 231. This is a validated parser-only progress slice.
