# Start autonomous compiler development loop

Use this prompt to invoke the compiler-autonomy skill and begin the FSM-driven development cycle.

## Prompt

```
Start autonomous compiler development loop.

Invoke the compiler-autonomy skill. Read:
- workflows/compiler_dev_fsm.md
- state/current_task.json
- state/project_state.json
- issues/index.md
- docs/11-shared-definitions.md
- docs/12-coding-standard.md

Follow the FSM:
SYNC → TRIAGE → TASK_SELECT → PLAN → IMPLEMENT → VERIFY → RETRO.

Use Anti-Stall Policy:
- Do not stop on first failure.
- On command failure, enter RECOVER mode.
- Retry transient failures once.
- Narrow failing gates before changing code.
- Preserve useful progress.
- Keep implementing Rust/compiler changes that make more fixtures pass.
- Commit every internally consistent forward step after narrow validation passes.
- Send or defer Discord reporting for each cycle and each local commit batch.
- If an issue cannot be completed, leave it open with evidence and continue to another Ready issue.
- Treat DONE, PROGRESS, and BLOCKED as valid cycle outcomes.
- Only move an issue to done when all close requirements are satisfied.
- External reporting failures must not erase local progress.

Primary goal:
maximize safe forward progress by making more fixtures pass while preventing false-done.
```

## When to use

- When you want the agent to work through issues autonomously following the FSM contract
- When starting a new development cycle on the compiler
- When resuming work after a context switch

## What happens

1. Agent reads the FSM contract from `workflows/compiler_dev_fsm.md`
2. Agent checks current state from `state/current_task.json` and `state/project_state.json`
3. Agent follows the state machine: SYNC → TRIAGE → TASK_SELECT → PLAN → IMPLEMENT → VERIFY → RETRO
4. Agent updates state files and writes cycle reports to `reports/runs/`
5. Agent keeps selecting the next safe task after DONE, PROGRESS, or BLOCKED until no safe task exists or the user stops the loop

## Operating Goal

The agent should behave like a continuous fixture-to-Rust implementation machine:

- Use failing fixtures, reference coverage, and issue acceptance criteria to choose the next smallest semantic gap.
- Prefer Rust implementation changes in compiler/runtime paths over weakening tests, lowering expectations, or marking unsupported cases as success.
- Add or update fixtures when they lock in real behavior that was implemented.
- Preserve test262 / fixture evidence and use it to generate follow-up issues when a gap is too large for the current cycle.
- Commit validated forward progress in small logical units.
- Report each cycle outcome to Discord, or save a deferred payload when reporting is unavailable.

Do not relax completion criteria to keep moving. Use PROGRESS or BLOCKED outcomes when DONE is not justified.

## Detailed Steps

### Preflight (SYNC state)

- Read `workflows/compiler_dev_fsm.md` to understand FSM contract
- Read `state/current_task.json` to check if a task is already in progress
- Read `docs/11-shared-definitions.md` for workstreams and gates
- Read `docs/12-coding-standard.md` for coding standards
- Read `issues/index.md` to see the current issue queue
- Run `scripts/manager check-agent-state` to validate state files

### Task Selection (TASK_SELECT state)

- If `current_task.json` is idle, select a Ready issue from `issues/index.md`
- Prioritize P0 issues, then P1, then P2
- Check dependencies are satisfied (issue must not be in Blocked queue)
- Update `current_task.json` with selected task details

#### Task Selection fallback

If no Ready issue is safely selectable:

1. Check whether the current task can be resumed
2. Check Blocked issues for dependencies that are now satisfied
3. Run issue health checks
4. Generate new issues from reference coverage
5. Update issue index
6. Commit issue/index changes
7. If still no task exists, write `reports/runs/<run_id>/no_task.md` and stop cleanly

Never stop at TASK_SELECT without writing why no task was selectable.

### Planning (PLAN state)

- Read the selected issue file from `issues/open/`
- Identify scope (allowed_files, forbidden_files)
- List acceptance criteria
- Define validation commands
- Create implementation plan

### Implementation (IMPLEMENT state)

- Make changes only within scope.allowed_files
- Do not modify forbidden files
- Follow coding standards from docs/12
- Run incremental validation as needed
- Implement the smallest Rust/compiler/runtime change that makes a real fixture or reference case advance
- Do not change expected fixture output unless the previous expectation is proven wrong by Node differential evidence or a documented spec decision
- Add regression coverage for implemented behavior when the issue touches semantics, lowering, runtime ABI, WASM emission, or CLI behavior

#### Fixture-driven Rust loop

Within a selected issue, repeat this inner loop while it remains in scope:

1. Pick one failing fixture, reference case, or acceptance criterion.
2. Reproduce it with the narrowest command available.
3. Classify the failure as parser, frontend semantics, IR/lowering, runtime ABI, backend-wasm, WASI/runtime, CLI, or test harness.
4. Change Rust code in the owning crate only when the failure is implementation-backed.
5. Run the narrow validation until it passes.
6. Run `scripts/manager fmt`.
7. Commit the validated implementation step if the worktree is internally consistent.
8. Record evidence in the issue note or cycle report.

Do not wait for the full issue to close before committing useful implementation progress. A commit may represent PROGRESS if it passes narrow validation and does not falsify broader gates.

### Verification (VERIFY state)

Run validation in layers. Do not jump directly to the broadest gate unless the narrow gates pass.

Validation order:

1. Format / syntax:
   - `scripts/manager fmt`

2. Narrow issue validation:
   - commands listed in the issue
   - targeted unit tests
   - targeted fixture tests
   - targeted CLI smoke tests
   - Node differential commands when semantics changed

3. Repo smoke:
   - `scripts/manager check-repo-smoke`
   - `scripts/manager check-agent-state`
   - `scripts/manager check-issue-health`

4. Full validation:
   - `scripts/manager nextest` with no filters

If a later layer fails after earlier layers passed:
- preserve the earlier PASS evidence
- classify the failure
- do not revert useful work automatically
- do not mark done
- enter RECOVER mode

Verify all acceptance criteria with evidence before moving an issue to done.

### Commit Policy

Commit local progress whenever a logical step is internally consistent and the relevant narrow validation passes.

Commit examples:
- Rust implementation makes a targeted fixture pass
- fixture or reference evidence is added for implemented behavior
- issue/index/report artifacts are updated after a cycle
- generated coverage issues are added when no Ready issue is available

Do not commit:
- known broken implementation without a clear progress note
- broad unrelated formatting mixed into implementation
- test expectation changes that hide a real compatibility gap
- secrets, webhook URLs, or transient local output

Commit messages should identify the issue or area, for example:

```bash
git commit -m "issue-123: implement array length fixture"
git commit -m "progress(runtime): pass targeted string fixture"
git commit -m "issues: record blocker for #123"
```

After a commit batch, write or update the run report and attempt Discord reporting.

### Anti-Stall Policy

The agent must prefer forward progress over stopping.

A failed command, missing optional file, flaky test, webhook failure, dirty worktree, or ambiguous issue state must not immediately stop the loop. Instead, enter RECOVER mode and perform the smallest safe recovery action.

#### RECOVER mode

Enter RECOVER mode when:
- a validation command fails
- a required file is missing
- the current issue is ambiguous or blocked
- the worktree is dirty in an unexpected way
- the selected issue cannot be completed within the current cycle
- external reporting such as Discord/webhook fails
- full-suite validation fails after local or issue-specific validation passed

RECOVER procedure:

1. Capture evidence:
   - command
   - exit code
   - stdout/stderr path
   - suspected cause
   - whether failure is local, repo-wide, flaky, environmental, or issue-specific

2. Try bounded recovery:
   - retry the command once if failure may be transient
   - run the smallest narrower command that isolates the failure
   - inspect recent local changes with `git diff --stat` and `git diff`
   - fix only if the fix is within the selected issue scope
   - run the narrow validation again

3. If recovery succeeds:
   - continue the FSM from the failed state

4. If recovery fails after 2 attempts:
   - do not mark the issue done
   - commit useful implementation progress only if it is internally consistent and passes narrow validation
   - add a blocked/progress note to the issue
   - write a recovery report under `reports/runs/<run_id>/recovery.md`
   - clear `current_task.json` only if no safe continuation exists
   - return to TASK_SELECT and pick another Ready issue if available

5. If no Ready issue exists:
   - run issue generation / coverage expansion
   - commit generated issues and index update
   - end with a report, not a silent stop

### Completion Levels

There are three valid outcomes for a cycle.

#### DONE

Use DONE only when:
- all acceptance criteria are satisfied
- required verification commands pass
- issue is moved from `issues/open/` to `issues/done/`
- index/state/reporting checks pass
- close commit is created

#### PROGRESS

Use PROGRESS when:
- implementation made meaningful forward progress
- narrow validation passes
- full close requirements are not yet satisfied
- the issue remains open
- evidence is recorded in the issue or report

PROGRESS is not failure. Continue the loop by selecting the next safe task or by continuing the same issue in the next cycle.

#### BLOCKED

Use BLOCKED when:
- the issue cannot proceed without dependency, missing design decision, missing tool, or repeated gate failure
- the blocker is written explicitly
- the issue is not moved to done
- a follow-up issue or blocker note is created

BLOCKED is not a reason to stop the whole loop if another Ready issue exists.

### External Reporting Policy

External reporting must not destroy local progress.

If Discord report or webhook delivery fails:
- save the report payload to `reports/runs/<run_id>/discord_payload.json`
- save the error to `reports/runs/<run_id>/reporting_error.log`
- retry once
- if retry fails, mark reporting as `DEFERRED`
- do not mark the issue done if project policy requires reporting for close
- do not discard implementation progress
- continue to another Ready issue if safe

Pre-push webhook may block push, but must not block local commits, reports, or further local task selection.

### Close (RETRO state)

**Prerequisites (all must be satisfied):**
- All acceptance criteria in the issue are satisfied with evidence in the repository
- Required verification / Close gate commands from the issue have been executed with exit 0 recorded
- No remaining tasks, STOP_IF, or blocked declarations remain in the issue body
- Implementation, verification, or documentation changes have been committed (not just mechanical checkbox updates)

**Close procedure:**

1. **Commit implementation changes** (if not already committed):
   - Run all validation commands from the issue
   - `scripts/manager fmt`
   - `scripts/manager nextest` (full suite, no filters)
   - Any fixture-specific tests (e.g., `iwasm fixture.wasm`)
   - Commit with descriptive message (e.g., `feat(area): implement NNN description`)
   - Record the commit hash

2. **Move issue to done**:
   - `git mv issues/open/<slug>.md issues/done/<slug>.md` (preserves history and path)
   - Update issue frontmatter: set Status to done
   - Add Close note with:
     - Date
     - Commit hash(es) providing evidence
     - Mapping of acceptance criteria to evidence
     - Verification commands executed and results

3. **Regenerate index and verify**:
   - `scripts/manager update-issue-index` to regenerate `issues/index.md`
   - `scripts/manager update-issue-index --check`
   - `scripts/manager check-issue-health`

4. **Final validation**:
   - `scripts/manager check-agent-state`
   - `scripts/manager check-repo-smoke`

5. **Commit close changes**:
   - Group into 1 commit or logically split into few commits (e.g., `chore(issues): close #NNN …`)
   - Ensure issue move and index update are in the same commit or logical sequence

6. **Cleanup and reporting**:
   - Clear `current_task.json` to idle state
   - Write cycle report to `reports/runs/<timestamp>/cycle_report.md`
   - Attempt `scripts/manager discord-report --run-id <run_id>`
   - If reporting fails, follow External Reporting Policy and mark reporting as `DEFERRED`
   - Update failure_patterns.md if new failure pattern discovered
   - Update review_checklist.md if new guard needed

7. **Continue**:
   - Return to TASK_SELECT after DONE, PROGRESS, or BLOCKED when another safe task exists
   - Continue implementing fixture-backed Rust changes until no safe task remains, then write a clean stop report

**False-done prevention:**
- Do not mark an issue as done without implementation-backed evidence
- Do not move issues with remaining `[ ]` checkboxes or open Status to done/
- Do not close issues based on text-only changes without verification
- Do not close issues with upstream dependencies still open

**Issue addition** (when Ready queue is low):

- Run reference-coverage with --detail flag:

  ```bash
  scripts/manager reference-coverage test262 --limit 500 --detail
  ```

- Pipe to gen-issues-from-coverage to auto-generate issues:

  ```bash
  scripts/manager reference-coverage test262 --limit 500 --detail | \
    scripts/manager gen-issues-from-coverage --suite test262
  ```

- Run `scripts/manager update-issue-index` to regenerate `issues/index.md`

- Commit changes:

  ```bash
  git add issues/ scripts/gen/issues-from-coverage.py
  git commit -m "feat(issues): add issues NNN-XXX from coverage"
  ```

**Coverage expansion** (when implementation targets decrease):
- Increase --limit in reference-coverage (e.g., 50 → 100 → 500 → 1000)
- Add new test suites if needed (tsc, tsgo)
- Auto-generate issues from expanded coverage
- Update issue index and commit

## Critical Requirements

**DONE close requirements are not satisfied until:**
- Full test suite passes (`scripts/manager nextest` with no filters)
- Issue file is moved from `issues/open/` to `issues/done/` with updated frontmatter
- `issues/index.md` is regenerated and verified
- All acceptance criteria are explicitly verified with evidence
- **Mechanical guards added** to `failure_patterns.md` or `review_checklist.md` if new patterns discovered
- **State files validated** with `scripts/manager check-agent-state`
- **Command outputs saved** to `reports/runs/<run_id>/stdout.log` and `stderr.log`
- **Discord report sent** with `scripts/manager discord-report --run-id <run_id>` (see retrospective-codify skill for format)
- **Pre-push webhook is mandatory**: `.githooks/pre-push` sends a webhook report after local gates pass and blocks push if delivery fails

Do not mark an issue as done without completing these steps. Failure to satisfy DONE close requirements must transition to PROGRESS, BLOCKED, or RECOVER instead of silently stopping the loop.

**Loop continuation requirements:**
- A cycle may end as DONE, PROGRESS, or BLOCKED.
- DONE is required only for moving an issue to `issues/done/`.
- PROGRESS and BLOCKED must include evidence and must leave the issue open.
- Discord/webhook failure may prevent DONE or push, but must not prevent local commits, reports, or selecting another safe task.

## Worktree/Branch Naming Convention

When starting a new task, create a worktree/branch following this pattern:

```
git worktree add ../ts2wasm-NNN-short-title NNNNNNNNNNNNN
# or
git checkout -b NNNNNNNNNNNNNN
```

Where:
- `NNN` is the issue ID (e.g., 012)
- `short-title` is a brief description (kebab-case, max 20 chars)
- `NNNNNNNNNNNNNN` is a 12-character timestamp or random suffix for uniqueness

Examples:
- `git worktree add ../ts2wasm-012-computed-prop 202604260728`
- `git checkout -b 012-computed-prop-202604260728`
