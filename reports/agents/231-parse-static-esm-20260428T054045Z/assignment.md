# Child Assignment: issue 231 parse static ES module declarations

- Child id: `231-parse-static-esm-20260428T054045Z`
- Worktree: `/home/wogikaze/wgkz/ts2wasm-231-parse-static-esm-20260428T054045Z`
- Branch: `agent/231-parse-static-esm-20260428T054045Z`
- Assigned issue: `231`

## Required first checks

Run `pwd`, `git status --short --branch`, and confirm this worktree/branch before editing. Do not touch the parent worktree.

## Scope

Implement the smallest safe parser-only slice for issue 231.

Preferred first slice:

- Add AST representation for side-effect import and named import/export declarations, with module specifier/name/span preservation.
- Convert the matching parser diagnostics into parser-success AST nodes for those supported forms.
- Keep namespace/default import and re-export diagnostics if implementing all forms is too large for one cycle.
- Do not implement resolver/module graph/lowering/backend semantics.

If the full issue is too large, commit validated PROGRESS with a subset and leave clear evidence in issue 231.

## Allowed files

- `crates/frontend/src/`
- `crates/cli/tests/` only for parser/build diagnostic fallout
- `fixtures/module-system/`
- `issues/open/231-parse-static-es-module-declarations.md`
- `issues/open/055-implement-import-export.md` only if progress evidence needs umbrella mention
- `reports/runs/231-parse-static-esm-20260428T054045Z/`

## Expected validation

- `cargo fmt --all --check`
- `cargo nextest run -p ts2wasm-frontend`
- focused CLI tests only if existing module diagnostics change
- `scripts/manager check-issue-health`
- `scripts/manager check-agent-state`

End with exactly one `PARENT_EVENT:` line.
