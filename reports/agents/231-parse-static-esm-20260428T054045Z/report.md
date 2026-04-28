# Child Worker Report: issue 231

Status: PROGRESS

Branch: `agent/231-parse-static-esm-20260428T054045Z`

## Scope Completed

- Added AST nodes for side-effect imports, named imports, and local named exports.
- Preserved module specifier strings, imported/local/exported names, and spans in parser tests.
- Converted supported parser forms from issue-055 parser errors into successful AST nodes.
- Kept namespace/default import, star re-export, named re-export, and dynamic import unsupported with issue-linked diagnostics.
- Added minimal downstream unsupported guards so workspace compile remains green without resolver/module graph/lowering semantics.

## Validation

```text
cargo fmt --all --check
PASS

cargo nextest run -p ts2wasm-frontend
PASS: 40 tests

cargo nextest run -p ts2wasm-cli static_named_import_reports_issue_055 static_named_export_reports_issue_055
PASS: 2 tests

cargo check --workspace
PASS

scripts/manager check-issue-health
PASS

scripts/manager check-agent-state
PASS
```

## Remaining

- Issue 231 remains open because default import, namespace import, re-export declarations, export default, declaration exports, and fixture conversion are not complete in this subset.
- No module graph, name resolution across files, lowering, backend, or runtime execution semantics were implemented.

## Commit

`d9df977` issue-231: parse static module declaration subset
