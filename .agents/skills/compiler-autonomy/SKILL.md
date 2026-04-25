---
name: compiler-dev-autonomy
description: "Use for autonomous / agent compiler-dev runs: FSM, current_task.json, verification reports, failure pattern DB, and re-prevention. Read workflows/compiler_dev_fsm.md at repo .agents/ root."
---

# Compiler dev autonomy

This skill is the **thin entry** for the autonomous build/test loop. The authoritative contract is large; it lives in the workflow + state files, not in this `SKILL.md` alone.

## Read order

1. `../../workflows/compiler_dev_fsm.md` — FSM, failure edges, done/forbidden, RETRO rules
2. `../../state/current_task.json` (and `../../state/project_state.json`) when work is in progress
3. `references/coding_standard.md` — project-bound rules
4. `references/review_checklist.md` — pre-verify gate
5. `references/failure_patterns.md` — FP-NNN failure DB (curated, do not bloat)

## State and artifacts

- JSON Schemas: `../../state/schemas/`
- Example `test_report`: `../../state/examples/test_report.json`
- Run output location: `reports/runs/<run_id>/test_report.json` (repo root)

## Related skills

- `ts2wasm-milestone` for roadmap slices
- `ts2wasm-gatekeeper-review` for merge gates
- `ts2wasm-scripts-workflow` for adding automation that implements a guard
