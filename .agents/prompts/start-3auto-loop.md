# Start 3-agent autonomous compiler development

Use this prompt to start a three-agent autonomous loop:

- Codex is the parent orchestrator and merge gatekeeper.
- Devin is child agent A.
- OpenCode is child agent B.

The goal is not to run three agents for appearance. The goal is to increase
verified issue throughput while keeping ownership boundaries, monitoring, and
merge control explicit.

## Prompt

```md
Start 3-agent autonomous compiler development.

Act as the Codex parent orchestrator.

Use `.agents/prompts/autonomous-parent-orchestrator.md` as the base contract.
Use `.agents/prompts/autonomous-child-worker.md` as the child behavior contract.

Run with exactly these roles:

- Codex parent: selects issues, splits work, creates worktrees, monitors child
  progress, reviews child commits, cherry-picks safe commits, runs final gates,
  updates issues/index, and reports outcomes.
- Devin child: works only in its assigned Devin worktree and branch.
- OpenCode child: works only in its assigned OpenCode worktree and branch.

Codex must do all cherry-pick, merge, issue close, and final gate decisions.
Children must not merge to `master`, push, force-push, or edit the parent
worktree.

Primary KPI: reduce `issues/open/` by closing verified issues or converting
stale implementation-ready work into accurate verification/triage/blocker
state with evidence.

Do not claim Devin or OpenCode were used unless their CLI processes were
actually started and their outputs were inspected.
```

## Required startup sequence

Run from the repository root.

1. Confirm parent state:

```bash
git status --short --branch
git rev-parse HEAD
git rev-parse origin/master
mise run update-issue-index -- --check
mise run check issues
```

1. Select two file-disjoint child assignments from `issues/index.md`.

Prefer initial assignments where each child can complete one of:

- small implementation slice with focused validation
- verification-ready issue cleanup
- reproduction normalization
- issue split with exact evidence

Avoid assigning both children to central backend/runtime files until the loop is
proven stable.

1. Create clean child worktrees from `origin/master`:

```bash
git worktree add ../ts2wasm-devin-<issue-id>-<short>-<yyyymmddhhmm> \
  -b agent/devin-<issue-id>-<short>-<yyyymmddhhmm> origin/master

git worktree add ../ts2wasm-opencode-<issue-id>-<short>-<yyyymmddhhmm> \
  -b agent/opencode-<issue-id>-<short>-<yyyymmddhhmm> origin/master
```

If `git worktree add` races on `.git/config`, do not guess. Inspect
`git worktree list --porcelain`, then either attach the existing branch to a
worktree or set upstream with `git branch --set-upstream-to=origin/master`.

1. Create a clean integration worktree:

```bash
git worktree add ../ts2wasm-agent-merge-<yyyymmddhhmm> \
  -b agent/integration-3auto-<yyyymmddhhmm> origin/master
```

The parent may cherry-pick into this integration branch first, then cherry-pick
accepted integration commits into `master`.

## Assignment files

For each child, write an assignment file under the child worktree:

```text
reports/agents/<agent_id>/assignment.md
```

The assignment must include:

- child id: `devin-<issue-id>` or `opencode-<issue-id>`
- worktree path
- branch name
- exact issue file path
- allowed files
- forbidden files
- required commands
- completion condition
- report format
- explicit instruction: do not merge, do not push, do not edit outside scope

Use this minimal assignment shape:

```md
# Assignment: <agent_id>

You are a child agent in the ts2wasm 3-agent loop.

Read first:

- AGENTS.md
- .agents/prompts/autonomous-child-worker.md
- <issue-file>

Worktree: <absolute-worktree-path>
Branch: <branch>
Issue: <issue-id> <title>

Allowed files:

- <path>

Forbidden files:

- <path>

Required validation:

- <command>

Done when:

- <observable condition>

Report exactly one final parent event:

PARENT_EVENT: DONE issue=<id> branch=<branch> commit=<hash> merge_request=yes
PARENT_EVENT: PROGRESS issue=<id> branch=<branch> commit=<hash-or-none> merge_request=no
PARENT_EVENT: BLOCKED issue=<id> branch=<branch> commit=<hash-or-none> reason=<short-reason>
PARENT_EVENT: FAILED issue=<id-or-none> branch=<branch> reason=<short-reason>
```

## Launching Devin

Preferred non-interactive command:

```bash
cd <devin-worktree>
devin --permission-mode auto --prompt-file reports/agents/<agent_id>/assignment.md
```

For short read-only audits, `-p` is acceptable:

```bash
cd <devin-worktree>
devin --permission-mode auto -p "Read-only audit. Do not edit files or run mutating commands. Inspect <issue-file> and report status, evidence, blockers, and next action."
```

Do not mark Devin as active until:

- the `devin` process has actually started
- its working directory is the child worktree
- the parent has recorded the command line in the cycle report

If Devin is silent for 15 minutes, ask for a status by continuing/resuming the
session if possible. If it remains silent for 45 minutes, stop using that child
for the current wave, inspect the worktree, and keep only useful committed
changes.

Useful Devin session commands:

```bash
devin list
devin --continue
devin --resume <session_id>
```

## Launching OpenCode

Preferred non-interactive command:

```bash
cd <opencode-worktree>
opencode run --file reports/agents/<agent_id>/assignment.md
```

For short read-only audits, pass the message as a positional argument to
`opencode run`. Do not use `opencode run --prompt`; `--prompt` is a top-level
TUI option, not a `run` option.

```bash
cd <opencode-worktree>
opencode run "Read-only audit. Do not edit files or run mutating commands. Inspect <issue-file> and report status, evidence, blockers, and next action."
```

If a specific model is needed:

```bash
opencode run -m <provider/model> --file reports/agents/<agent_id>/assignment.md
```

Do not mark OpenCode as active until:

- the `opencode run` process has actually started
- its working directory is the child worktree
- the parent has recorded the command line in the cycle report

Useful OpenCode session commands:

```bash
opencode session
opencode run --continue
opencode run --session <session_id>
opencode export <session_id>
```

## Supervision loop

Every 15 minutes for each active child, collect:

```bash
git -C <child-worktree> status --short --branch
git -C <child-worktree> log --oneline origin/master..HEAD
git -C <child-worktree> diff --stat
```

Classify child state:

- ACTIVE: process is running or producing output
- MERGE_REQUEST: final event says merge requested and there is at least one commit
- PROGRESS: useful commit exists but issue is not closeable
- BLOCKED: blocker is specific and evidence-backed
- STUCK: no output/commit/report after the timeout
- VIOLATION: child edited outside allowed files, weakened tests, touched parent worktree, or tried to merge/push

Recovery rules:

- For STUCK: inspect logs and worktree; preserve useful commits only.
- For VIOLATION: reject the branch unless the useful commit can be isolated by
  cherry-pick without the violation.
- For BLOCKED: update or split issues only with exact evidence.
- For PROGRESS: cherry-pick only if the commit is internally consistent and
  improves a verified issue state.

## Parent merge protocol

Children never merge. Codex parent integrates.

1. Inspect child commits:

```bash
git -C <child-worktree> status --short --branch
git -C <child-worktree> log --oneline origin/master..HEAD
git -C <child-worktree> diff --stat origin/master..HEAD
git -C <child-worktree> show --name-only --oneline HEAD
```

1. Reject immediately if changed files violate the assignment.

1. Cherry-pick into the integration worktree:

```bash
cd <integration-worktree>
git cherry-pick <child-commit>
```

1. If `issues/index.md` conflicts, regenerate it instead of hand-editing the
generated region:

```bash
mise run update-issue-index
git add issues/index.md
git cherry-pick --continue
```

1. Run validation in the integration worktree:

```bash
mise run update-issue-index -- --check
mise run check issues
cargo fmt --all --check
```

Run issue-specific tests from the child assignment. Run `cargo nextest run` when
closing an issue, changing compiler/runtime behavior, or before push.

1. Cherry-pick accepted integration commits into the parent branch:

```bash
cd <parent-worktree>
git cherry-pick <integration-commit>
```

1. Re-run parent gates:

```bash
mise run update-issue-index -- --check
mise run check issues
cargo fmt --all --check
```

Do not push if the parent worktree has unrelated, unclassified dirty files.
Do not bypass pre-push hooks.

## Cycle report

At the end of every wave, write a local report under:

```text
reports/runs/<run_id>/cycle_report.md
```

The report must include:

- commands used to launch Devin and OpenCode
- child worktree paths and branches
- child final events
- accepted commits
- rejected commits
- validation commands and results
- remaining blockers
- next assignments

Discord report content must be Japanese and brief. Send or defer it according
to `docs/16-commit-and-push-policy.md`.

## Minimum successful wave

A wave is successful only if all are true:

- both `devin` and `opencode` were actually launched, or a launch failure was
  recorded with the exact command and stderr
- at least one child produced a useful final event or verified blocker
- Codex reviewed the child output
- Codex cherry-picked only safe commits, or explicitly rejected all child output
- validation ran after integration
- issue/index state is accurate

If this minimum is not met, report the wave as failed and fix the loop before
assigning implementation-heavy work.

```

## Common commands

Read-only Devin audit:

```bash
devin --permission-mode auto -p "Read-only audit. Do not edit files or run mutating commands. Inspect issue <id> and report status, evidence, blockers, and next action."
```

Read-only OpenCode audit:

```bash
opencode run "Read-only audit. Do not edit files or run mutating commands. Inspect issue <id> and report status, evidence, blockers, and next action."
```

Implementation Devin run:

```bash
devin --permission-mode auto --prompt-file reports/agents/<agent_id>/assignment.md
```

Implementation OpenCode run:

```bash
opencode run --file reports/agents/<agent_id>/assignment.md
```

## Prompt files

- `.agents/prompts/autonomous-parent-orchestrator.md` — parent worktree orchestration prompt.
- `.agents/prompts/autonomous-child-worker.md` — child implementation prompt.
- `.agents/prompts/start-autonomous-loop.md` — Codex/subagent launcher prompt.
- `.agents/prompts/start-3auto-loop.md` — Codex + Devin + OpenCode launcher prompt.
