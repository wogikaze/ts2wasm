# Agent Outcome: issue 022 coverage evidence

Status: DONE

Implemented additive coverage evidence representation:

- `artifacts/coverage/results/test262.json` keeps the canonical sorted ramp result and now carries a `test262 semantic-core seeds` evidence row.
- `scripts/gen/coverage-matrix.py` renders canonical result rows plus optional additive evidence rows.
- `artifacts/coverage/reference-coverage-matrix.md` now shows both the Gate D sorted ramp row and the Gate E selected-seed row.
- `docs/15-coverage-matrix.md` documents that selected subset rows do not replace canonical ramp rows.
- Issue 022 was moved to `issues/done/` and `issues/index.md` was regenerated.

Validation passed:

- `python scripts/manager.py reference-coverage test262 --limit 100 --detail`
- `python scripts/manager.py reference-coverage test262 --paths-file scripts/data/test262-semantic-core-seeds.txt --detail`
- `scripts/manager update-coverage-matrix --check`
- `scripts/manager update-issue-index --check`
- `scripts/manager check-issue-health`
- `scripts/manager check-repo-smoke`
- `scripts/manager check-agent-state`
- `cargo fmt --all --check`
- `scripts/manager nextest`
