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
