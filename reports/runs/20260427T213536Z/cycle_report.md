# Cycle Report: issue-202 RegExp Literal

Run id: 20260427T213536Z
Status: PROGRESS
Implementation commit: 27a551a

## Work Completed

- Accepted basic RegExp literal tokens in expression position by mapping the supported subset to the existing string expression/lowering path.
- Preserved raw literal text for `/pattern/flags` so current `console.log` output matches Node for the covered subset.
- Added issue-linked `UnsupportedSyntax` diagnostics for unsupported and duplicate flags.
- Added supported and unsupported fixtures under `fixtures/core-semantics/`.

## Evidence

Targeted checks passed:

- `cargo test -p ts2wasm-cli regexp --lib`
- `cargo test -p ts2wasm-cli --test ir_lowering lowering_routes_regexp_literal_to_string_subset`
- `cargo test -p ts2wasm-cli --test m2_node_diff regexp_unsupported_flag_fixture_reports_issue_202`
- `cargo test -p ts2wasm-cli --test m2_node_diff regexp_literal_fixture_matches_node_output_under_iwasm`
- `cargo fmt --all --check`

Manual fixture check:

- Node output and iwasm output matched for `fixtures/core-semantics/regexp-literal.ts`:
  - `/abc/i`
  - `/a*/g`
  - `/a\/b/`
  - `/[a/]/`

## Blockers

- Reference coverage could not verify unsupported bucket reduction because `reference/test262` is missing.
- Full `cargo nextest run` is not clean due to existing unrelated failures listed in `test_report.json`.
- Issue health gates are red on existing stale issue path references to `crates/cli/src/backend`.

## Next Step

Parent can either merge this progress slice or supply a worktree with initialized reference sources so the `unsupported_features.regexp-literal` reduction can be measured and issue 202 can be considered for DONE.
