# Assignment: issue 227 type reference directive closure

Agent ID: codex-227-type-ref-close-20260428T024058Z
Worktree: /home/wogikaze/wgkz/ts2wasm-227-type-ref-close-20260428T024058Z
Branch: agent/227-type-ref-close-20260428T024058Z
Issue: 227 (issues/open/227-implement-type-reference-directive-resolution.md)

## Scope

Perform closure-oriented verification for issue 227. If every acceptance criterion is already satisfied by existing implementation and precise diagnostic/suppression behavior, move issue 227 to issues/done/, update completion evidence and issue index, and commit. If full type-package resolution is still required by acceptance, do not mark done; implement only a narrow safe 227 regression if needed or record PROGRESS/BLOCKED with evidence.

Allowed files:
- issues/open/227-implement-type-reference-directive-resolution.md
- issues/done/**
- issues/index.md
- fixtures/typescript-directives/**
- crates/frontend/src/** only if a verified 227 acceptance gap remains
- crates/cli/tests/** only for 227 regression coverage
- scripts/lib/feature-labels.sh only if reference classification is demonstrably stale for 227
- reports/agents/**
- reports/runs/**

Forbidden files:
- docs/**
- backend/runtime implementation files
- unrelated issue files except index regeneration

## Validation Plan

1. Read .agents/prompts/autonomous-child-worker.md and issue 227 acceptance criteria.
2. Inspect current branch/worktree state and identify existing 227-specific frontend/CLI tests with rg.
3. Verify each acceptance criterion explicitly against source, fixtures, and tests.
4. Run cargo fmt --all --check.
5. Run the identified 227-specific frontend/CLI tests.
6. Run TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage tsgo --path-filter processingDiagnostic --detail.
7. Run the issue's tsgo --limit 120 command if feasible.
8. If closing, run cargo nextest run, scripts/manager update-issue-index, scripts/manager check-issue-health, and scripts/manager check-agent-state.

## Webhook and Reporting Plan

Save a cycle report under reports/runs/<timestamp>-227-type-ref-close/. If a Discord webhook is configured and available through local project conventions, send the completion/progress payload. If unavailable, save a deferred payload in the report directory and continue.

## Merge Protocol

Stay on branch agent/227-type-ref-close-20260428T024058Z in the assigned worktree. Do not revert or overwrite work from other agents. Commit only validated closure/progress changes. If DONE, include the moved issue, updated issue index, evidence/report files, and any allowed regression changes. End with exactly one PARENT_EVENT line indicating DONE, PROGRESS, or BLOCKED with branch and commit hash.
