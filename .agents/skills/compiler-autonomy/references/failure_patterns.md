# Failure pattern DB

This is a **bounded, curated** set of known failure classes. It replaces unbounded `llm_skills.md` dumps.

- Each pattern has a stable `FP-NNN` id.
- Prefer **short** entries; link to an issue, fixture path, or script if detail is long.
- When a pattern is hit in prod-like runs, the retro must add at least one **mechanical** guard: checklist line, fixture, script/lint, or gate — not prose alone.

## FP-001: Model JS `+` as numeric addition everywhere

**Trigger**

- `BinaryExpression` with operator `'+'`
- Operands are **not** statically proven to be `number`

**Check**

- String concatenation
- `ToPrimitive` / `ToString` paths
- `ToNumber` coercions
- Unknown or dynamic value kinds

**Required action**

- Lower only the number-proven fast path, or
- Emit an unsupported diagnostic, or
- Runtime dispatch (when in scope) — as defined by the task, not by convenience.

**Guards to add on recurrence**

- [ ] Item in `review_checklist.md` for `+` sites
- [ ] Regression fixture covering mixed-type or dynamic `+` if not already present
- [ ] If applicable, a gate script or type predicate assertion in IR lowering

_Add the next real incident as `FP-002+`; do not keep placeholder stubs._
