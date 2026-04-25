# Compiler dev FSM (autonomous mode)

This document is the **behavioral contract** between the repo and an agent loop. It is not product documentation; product truth stays in `docs/`. Work orders stay in `issues/`.

## Layout (filesystem)

```text
.agents/
  state/
    project_state.json
    current_task.json
    last_run.json
    baseline.json
    decision_log.md
    schemas/
    examples/
  skills/
    compiler-autonomy/
      SKILL.md
      references/
        coding_standard.md
        failure_patterns.md
        review_checklist.md
  workflows/
    compiler_dev_fsm.md
  README.md
issues/
  open/
  done/
  index.md
reports/
  runs/
```

## State artifacts (I/O contract)

| Artifact | Role |
|----------|------|
| `.agents/state/project_state.json` | FSM name, counters (e.g. fast-verify streak), optional `active_task_id` |
| `.agents/state/current_task.json` | Single active task: scope, acceptance, `commands.fast` / `commands.full`, risk |
| `.agents/state/last_run.json` | Pointer to the latest `test_report` and coarse outcome |
| `.agents/state/baseline.json` | Optional perf numbers for `test_report.perf` deltas |
| `.agents/state/decision_log.md` | **Append-only** non-chat decisions (gates, scope cuts) |
| `.agents/state/schemas/*.json` | JSON Schemas for `current_task` and `test_report` |
| `reports/runs/<run_id>/test_report.json` | Machine-readable verification result for a run |

`current_task` must validate against `.agents/state/schemas/current_task.schema.json`.  
`test_report` must validate against `.agents/state/schemas/test_report.schema.json` (see `.agents/state/examples/test_report.json`).

## FSM (happy path)

```text
SYNC
  ↓
TRIAGE
  ↓
TASK_SELECT
  ↓
PLAN
  ↓
PLAN_REVIEW_GATE
  ↓
IMPLEMENT
  ↓
SELF_REVIEW_GATE
  ↓
VERIFY_FAST
  ↓ pass
VERIFY_FULL
  ↓ pass
CLOSE_OR_SPLIT
  ↓
RETRO
  ↓
SYNC
```

## Failure branches (explicit)

### `VERIFY_FAST` fail

- Go to **DIAGNOSE** (root cause, minimal hypothesis).
- **PATCH_SAME_TASK** (stay inside `current_task.scope` unless a formal scope update is decided and logged).
- Re-run **VERIFY_FAST** from a clean, reproducible command list.

**3 consecutive** `VERIFY_FAST` failures (track `project_state.verify_fast_streak_fails` or equivalent):

- **SPLIT_TASK** — break the work; write a **blocker** issue; reset streak counter.
- Return to **TASK_SELECT** with `current_task` cleared or set to the new, smaller work unit.

### `VERIFY_FULL` fail

- Classify: `regression` / `flaky` / `perf` / `unsupported` (or project-specific).
- **Either** patch in scope **or** file a follow-up issue; do not “force done” on ambiguous failures.

`VERIFY_FULL` may be **skipped** only if `current_task` (or the linked `issues/open/*` file) states an explicit, reviewed reason, and the skip is written into `test_report` metadata or `last_run.json`.

### `PLAN_REVIEW_GATE` fail

- **Rewrite the plan, not the code** — return to `PLAN` and adjust scope/acceptance/risk.
- No implementation until the gate passes.

### `SELF_REVIEW_GATE` fail

- **Patch** against `.agents/skills/compiler-autonomy/references/review_checklist.md` before any verification.
- If the problem is a missing guardrail, add a checklist item, fixture, or script in the same task *only* if in scope; otherwise file an issue and block.

## Close conditions (“Done is strict”)

All of the following:

- [ ] `current_task.acceptance` is satisfied and recorded
- [ ] `VERIFY_FAST` passed for the final patch set
- [ ] `VERIFY_FULL` passed, **or** a documented, approved skip reason (written in the issue and reflected in `last_run` / `test_report` meta)
- [ ] `fixtures.newly_failed` in `test_report` is an **empty** array
- [ ] **Docs drift** check: no unintended edits in final-state docs; `docs/current-state` / issue sync per `issues/README.md`
- [ ] The corresponding **issue** (if any) records commands run and their outcomes
- [ ] The issue is moved to `issues/done/`, and `issues/index.md` is updated (regenerate with `scripts/manager update-issue-index` if used)

**Forbidden (stability / anti-cheat)**

- [ ] No single task that simultaneously changes parser, IR, runtime, docs, and fixture layout without an explicit, split plan
- [ ] No turning failing tests into `skip` to reach Done (except an explicit, narrow exception with a follow-up issue)
- [ ] No closing on benchmark/perf regression with “we’ll ignore for now” without a follow-up and recorded baseline
- [ ] No rewriting `architecture` / high-level design docs to retroactively match code in the same implement-only task
- [ ] **No** appending only abstract “lessons” to a catch-all free-form file — use `.agents/skills/compiler-autonomy/references/failure_patterns.md` and convert at least one lesson into a **mechanical** guard (see **RETRO**)

## RETRO: “re-prevention,” not “learning journal”

Phase 4 / RETRO is **re-prevention patch construction**, in this order of preference:

1. Add a concise entry to `compiler-autonomy/references/failure_patterns.md` (stable id, trigger, check, action).
2. Add or tighten an item in `compiler-autonomy/references/review_checklist.md` if a human check should catch it next time.
3. If applicable, add a **regression fixture** under `fixtures/` (follow the repo’s fixture workflow).
4. If applicable, add **automation**: lint, ast-grep rule, or gate script — the strongest guard.

At least **one** mechanical output (2–4) is required; prose-only retros fail the RETRO step.

## Meta-improvement limits

- `failure_patterns.md` is curated: merge duplicates, retire stale FPs, avoid unbounded growth.
- `decision_log.md` stays **short per entry**; move deep discussion to `issues/`.
- Baseline/perf history belongs in `baseline.json` + per-run `test_report`, not free-form text files.

## Related

- `issues/README.md` — work order lifecycle, completion bar
- `AGENTS.md` — repo commands and default verification expectations
