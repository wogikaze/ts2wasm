# Cycle Report: 061 Date Annex B diagnostics

Run id: `061-date-annexb-diagnostics-20260428T051924Z`
Branch: `agent/061-date-annexb-diagnostics-20260428T051924Z`
Implementation commit: `cb72d09`
Status: PROGRESS

## Scope

Implemented the assigned narrow continuation for issue 061: precise unsupported diagnostics for Annex B legacy Date methods in the deterministic Date epoch slice.

Added fixtures:

- `fixtures/builtins-and-io/date-annexb-get-year-unsupported.ts`
- `fixtures/builtins-and-io/date-annexb-set-year-unsupported.ts`
- `fixtures/builtins-and-io/date-annexb-to-gmt-string-unsupported.ts`

Updated references:

- `crates/ir/src/lowered.rs`
- `crates/cli/tests/m2_node_diff.rs`
- `issues/open/061-implement-date.md`

## Evidence

- `cargo fmt --all --check`: PASS
- `cargo nextest run -p ts2wasm-cli --test m2_node_diff date_annex_b_fixtures_report_issue_061`: PASS, 1 passed
- `cargo nextest run -p ts2wasm-cli date --no-fail-fast`: PASS, 6 passed
- `cargo nextest run -p ts2wasm-cli`: PASS, 208 passed, 4 skipped
- Direct CLI diagnostic evidence:
  - `date-annexb-get-year-unsupported.ts`: exit 1, `[UnsupportedSyntax] issue-061: Date.prototype.getYear ... at 12..33`
  - `date-annexb-set-year-unsupported.ts`: exit 1, `[UnsupportedSyntax] issue-061: Date.prototype.setYear ... at 37..54`
  - `date-annexb-to-gmt-string-unsupported.ts`: exit 1, `[UnsupportedSyntax] issue-061: Date.prototype.toGMTString ... at 12..37`
- `scripts/manager check-repo-smoke`: PASS
- `scripts/manager check-issue-health`: PASS
- `scripts/manager check-agent-state`: PASS
- Fixture scans:
  - `sg run --lang rust -p 'TestRecord { $$$ }' crates/cli/tests crates/shared/src`: PASS
  - `sg run --lang rust -p 'assert_fixture_matches_node($$$)' crates/cli/tests`: PASS
  - `rg -n 'date-annexb|getYear|setYear|toGMTString' ...`: PASS, only new fixtures and test references

## Residual Risk

`cargo nextest run` for the full workspace failed before completion on two pre-existing-looking backend GC-root assertion tests unrelated to the Date diagnostic slice:

- `ts2wasm-backend-wasm tests::function_locals_are_mirrored_into_activation_gc_root_frames`
- `ts2wasm-backend-wasm tests::top_level_locals_are_mirrored_into_gc_root_table`

The focused backend rerun of those two tests reproduced the same failures. This child did not modify backend GC-root logic.

## Reporting

`scripts/manager discord-report --run-id 061-date-annexb-diagnostics-20260428T051924Z` failed twice because `DISCORD_WEBHOOK_URL` is not configured. Deferred payload and error files are saved in this run directory.
