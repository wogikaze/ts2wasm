# issue 022 test262 selection-mode progress

Run id: 20260428T001538Z-022-test262-selection
Branch: agent/022-test262-selection-20260428T093500Z
Outcome: PROGRESS

## Changes

- Added deterministic selection options to `scripts/run/reference-coverage.py`:
  - `--paths-file PATH` reads repo-relative or suite-relative paths from a curated list.
  - `--path-filter TEXT` filters by repo-relative path substring and may be repeated.
- Added `scripts/data/test262-semantic-core-seeds.txt` with 60 official test262 files verified as currently runnable build/semantic candidates.
- Updated issue 022 with selection-mode evidence while keeping the issue open.

## Evidence

- `python scripts/manager.py reference-coverage test262 --limit 0`: pass, zero-run preflight.
- `python scripts/manager.py reference-coverage test262 --paths-file scripts/data/test262-semantic-core-seeds.txt --detail`: pass; `executed=60`, `build_pass=60`, `semantic_pass=60`, `unsupported=0`, `fail=0`, `blocked=0`, `semantic_enabled=1`.
- `python scripts/manager.py reference-coverage test262 --path-filter language/comments --limit 2 --json`: pass; selector plumbing exercised with JSON output.
- `scripts/manager update-coverage-matrix --check`: pass; artifacts unchanged and matrix up to date.
- `scripts/manager update-issue-index --check`: pass.
- `scripts/manager check-issue-health`: pass.
- `scripts/manager check-repo-smoke`: pass.
- `scripts/manager check-agent-state`: pass.
- `scripts/manager fmt`: pass.
- `scripts/manager nextest`: pass; 244 passed, 4 skipped.
- `scripts/manager discord-report --run-id 20260428T001538Z-022-test262-selection < reports/runs/20260428T001538Z-022-test262-selection/cycle_report.md`: blocked by missing `DISCORD_WEBHOOK_URL`; deferred locally in `reports/runs/20260428T001538Z-022-test262-selection/discord-deferred.md`.

## Matrix decision

Coverage artifacts were not updated. The existing matrix records the sorted `--limit 100` ramp with `executed=100`; replacing it with the selected subset would reduce the canonical Gate D evidence to `executed=60`. The new selected-run evidence is recorded in this report and issue 022 as Gate E progress.
