# Cycle report: 231 final close audit

Run ID: `231-final-close-audit-20260428T084159Z`
Branch: `agent/231-final-close-audit-20260428T084159Z`
Issue: `231`
Outcome: DONE
Close commit: branch HEAD, reported in `PARENT_EVENT`

## Scope

Audited issue 231 after the export-class unsupported guard merge and closed it only after every acceptance criterion had direct evidence.

## Acceptance evidence

- Static import/export declarations listed in scope parse into explicit AST nodes: verified by frontend AST variants and passing parser tests for side-effect import, named import, default import, default+named import, default+namespace import, namespace import, named export, named re-export, star re-export, namespace re-export, `export const`, and expression `export default`.
- Parser tests assert specifier/name/span preservation: verified by `cargo nextest run -p ts2wasm-frontend`, including the module parser tests that assert source specifier spans, local/imported/exported names, declaration spans, and default marker spans.
- Existing issue-055 module guard fixtures remain narrower unsupported cases at the compiler boundary: verified by the 13 targeted `ts2wasm-cli::m9_modules static_*_reports_issue_055` tests.
- Unsupported forms still produce issue-linked diagnostics: verified by frontend tests for dynamic import, default function/class exports, variable export, and class export, plus CLI coverage for `static-class-export-unsupported.ts`.
- No frontend parser regression: `cargo nextest run -p ts2wasm-frontend` passed.

## Validation

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-frontend: PASS (48 tests)
cargo nextest run -p ts2wasm-cli static_named_import_reports_issue_055 static_side_effect_import_reports_issue_055 static_namespace_import_reports_issue_055 static_default_import_reports_issue_055 static_combined_named_import_reports_issue_055 static_combined_namespace_import_reports_issue_055 static_named_export_reports_issue_055 static_re_export_reports_issue_055 static_named_re_export_reports_issue_055 static_namespace_re_export_reports_issue_055 static_declaration_export_reports_issue_055 static_default_export_reports_issue_055 static_class_export_reports_issue_055: PASS (13 tests)
scripts/manager check-issue-health: PASS
scripts/manager check-agent-state: PASS
scripts/manager update-issue-index: PASS
scripts/manager update-issue-index --check: PASS
scripts/manager check-issue-index: PASS
cargo nextest run: PASS (358 tests, 4 skipped)
```

Logs are under `reports/runs/231-final-close-audit-20260428T084159Z/logs/`.

## Files changed

- Moved `issues/open/231-parse-static-es-module-declarations.md` to `issues/done/231-parse-static-es-module-declarations.md`.
- Marked issue 231 done and added completion evidence.
- Updated umbrella issue 055 links so health checks point at the done issue path.
- Regenerated `issues/index.md`.

## Remaining work

Module graph construction, cross-file name resolution, lowering, backend emission, and execution coverage remain intentionally out of scope and tracked by issues 232, 233, and 234.
