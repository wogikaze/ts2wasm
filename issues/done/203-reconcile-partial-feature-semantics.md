---
id: 203
title: "Reconcile partial feature semantics and placeholder completions"
type: cleanup
area: docs/issues
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
---

## Summary

Some completed issue files record syntax-only or placeholder implementations while docs previously marked the same features as either fully unimplemented or fully done. Track the remaining semantic work explicitly so future gates do not treat placeholder behavior as compatibility progress.

## Problem

Several `issues/done/` entries contain phrases such as "placeholder", "deferred to follow-up", or "new issue needed". Without a follow-up work item, the done queue can overstate compatibility and docs can drift from implementation evidence.

Known partial areas:

- `instanceof` full prototype-chain semantics (`issues/done/030-implement-instanceof-operator.md`)
- `switch` fall-through semantics (`issues/done/033-implement-switch-statement.md`)
- labeled `break` / `continue` (`issues/done/035-implement-break-continue.md`)
- arrow function closure + lexical `this` semantics (`issues/done/036-implement-arrow-function.md`)
- `this` binding placeholder verification (`issues/done/037-implement-this-binding.md`)
- rest parameter argument collection (`issues/done/038-implement-rest-parameters.md`)
- template literal `${...}` interpolation (`issues/done/041-implement-template-literals.md`)
- string method placeholder implementations (`issues/done/042-implement-string-methods.md`)
- `Math.random` deterministic placeholder / capability policy (`issues/done/053-implement-math.md`)
- abstract equality coercion (`issues/done/058-implement-equality-operators.md`)

## Desired final state

Each partial feature is either implemented with semantic differential evidence or split into a dedicated open issue with acceptance criteria and validation commands. Final-state docs and `current-state.md` distinguish "implemented", "basic", and "partial" consistently.

## Scope

In scope:

- [x] Audit the listed done issues for placeholder or deferred semantics.
- [x] Split any large remaining work into dedicated feature issues.
- [x] Update `docs/language-reference/javascript-features.md` and related semantics docs with exact status.
- [x] Add or update validation commands that prove semantic behavior, not only parsing/build success.
- [x] Add a mechanical check or checklist rule if a done issue says "new issue needed" without an open follow-up.

Out of scope:

- [x] Implementing the compiler/runtime behavior in this cleanup issue.

## Affected paths

Expected:

- `issues/open/`
- `issues/done/`
- `issues/index.md`
- `docs/language-reference/javascript-features.md`
- `docs/05-compatibility-and-semantics.md`
- `current-state.md`

Do not touch:

- `crates/`

## Acceptance criteria

- [x] No done issue entry with placeholder/deferred semantics lacks a linked open follow-up or documented scope decision.
- [x] Docs do not claim full implementation for placeholder behavior.
- [x] `mise run update-issue-index -- --check` passes.
- [x] `mise run check-issue-health` passes.
- [x] `mise run check-repo-smoke` passes.

## Validation

Required commands:

```sh
mise run update-issue-index -- --check
mise run check-issue-health
mise run check-repo-smoke
```

Impacted commands:

```sh
rg -n "placeholder|deferred to follow-up|new issue needed" issues/done
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated `docs/language-reference/javascript-features.md`
- [x] updated `docs/05-compatibility-and-semantics.md`

Current state:

- [x] updated `current-state.md`

Follow-up issues:

- [x] Existing open issues 207-216 cover the audited partial semantics. No duplicate issues were created.

## Completion evidence

Commits:

- this docs/issues cleanup commit

Validation result:

```text
command: rg -n "placeholder|deferred to follow-up|new issue needed" issues/done
result: all semantic placeholder/deferred matches in the audited done issues link to issues/open/207-216; non-semantic numeric matches are historical test counts
date: 2026-04-28

command: mise run update-issue-index
result: pass
date: 2026-04-28

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-28

command: mise run check-issue-health
result: pass
date: 2026-04-28

command: mise run fmt
result: pass
date: 2026-04-28

command: mise run check-repo-smoke
result: pass
date: 2026-04-28

command: mise run check-agent-state
result: pass
date: 2026-04-28
```

Remaining risks:

- Issues 207-216 still require implementation and Node differential evidence before those partial features count as semantic parity.
