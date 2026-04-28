# Start autonomous multi-worktree compiler development

Use this prompt to start the parent orchestrator for multi-worktree autonomous compiler development.

## Prompt

```md
Start autonomous multi-worktree compiler development.

Act as the parent orchestrator.

Use `.agents/prompts/autonomous-parent-orchestrator.md`.

Maximize safe subagent/worktree expansion.
Keep child agents continuously supplied with issue lists.
Each child must work in its own worktree, close or progress all assigned issues, commit validated work, send or defer webhook reports, and request merge from the parent.

Discord reporting is part of the loop, not an afterthought:

- Keep Discord reports very brief: status, issue IDs, validation, blockers, next action only.
- Write Discord report content in Japanese; keep commands, paths, and issue IDs as literals only.
- Do not leave sections as `未記入`; `discord-report` rejects placeholder-heavy reports.
- After every child outcome, require a `reports/runs/<run_id>/cycle_report.md` or saved `discord_payload.json`.
- Send reports with `scripts/manager discord-report reports/runs/<run_id>/cycle_report.md --run-id <run_id>`.
- `discord-report` automatically splits oversized messages into two sends; do not manually skip reporting because of Discord limits.
- If sending fails, save the payload/error under `reports/runs/<run_id>/`, retry once, mark `DEFERRED` if it still fails, and continue.
- Before merging a child or assigning the next wave, the parent must record whether reporting was `SENT` or `DEFERRED`.
- At the end of every parent cycle, send or defer the parent cycle report the same way.
- `reports/` is local and git-ignored; do not commit report artifacts.

Do not stop when one issue blocks.
Do not stop when one child fails.
Do not stop when Ready issues run out before generating more reference-backed issues.
If issues are exhausted, generate more from reference coverage and continue.
If no safe work can be generated, write a clean stop report.

Use subagents for high-load investigation, design, implementation, testing, and review.
Do not use subagents as search engines.

End each parent cycle with:
ORCHESTRATOR_STATUS: CONTINUE
or a justified clean stop status.
```

## Prompt files

- `.agents/prompts/autonomous-parent-orchestrator.md` — parent worktree orchestration prompt.
- `.agents/prompts/autonomous-child-worker.md` — child implementation prompt.
- `.agents/prompts/start-autonomous-loop.md` — short launcher prompt.
