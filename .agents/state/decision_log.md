# Decision log (append-only)

**Rules**

- Newest entries at the **top** of the `## Log` section.
- One decision per block: `when`, `state`, `decision`, `rationale`, `reversible?`, `follow-up` (issue id or `none`).

**Do not** use this file for implementation notes, chat transcripts, or unbounded “lessons” — that belongs in `issues/` or a structured failure entry in `skills/failure_patterns.md` with a machine guard.

## Log

<!-- example:

### 2026-04-26 — plan gate on WB-042

- **When**: 2026-04-26T10:00:00Z
- **State**: PLAN_REVIEW_GATE
- **Decision**: Approve plan with scope narrowed to number-proven operands only
- **Rationale**: String concat requires separate lowering path; out of this task
- **Reversible?**: yes
- **Follow-up**: new issue for dynamic `+` dispatch

-->
