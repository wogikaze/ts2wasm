# issue 060 progress report

Date: 2026-04-28
Branch: `agent/060-unsupported-classification-20260428T102000Z`
Outcome: PROGRESS

## Scope

Classified visible `unknown-unsupported` reference coverage cases without changing compiler implementation files.

## Changes

- Added classifier labels in `scripts/lib/feature-labels.sh` and the Python reference coverage classifier.
- Added `TS2WASM_REFERENCE_ROOT` support so isolated worktrees can run coverage against external reference checkouts.
- Refreshed coverage artifacts for test262 and added validated tsc/tsgo result artifacts.
- Recorded issue evidence in `issues/open/060-investigate-unknown-unsupported-cases.md`.

## Classification evidence

```text
test262 --limit 100: unknown-unsupported=0
test262 --limit 200: unknown-unsupported=0
tsc --limit 100: unknown-unsupported=0
tsgo --limit 82: unknown-unsupported=0
```

Detailed JSON outputs are saved under:

- `reports/runs/issue060-unsupported-classification-20260428T102000Z/test262-limit100.json`
- `reports/runs/issue060-unsupported-classification-20260428T102000Z/test262-limit200.json`
- `reports/runs/issue060-unsupported-classification-20260428T102000Z/tsc-limit100.json`
- `reports/runs/issue060-unsupported-classification-20260428T102000Z/tsgo-limit82.json`

## Remaining work

Issue 060 remains open because the full acceptance criteria require exhausting all unknown-unsupported cases across broader reference coverage, not only the validated windows in this slice.
