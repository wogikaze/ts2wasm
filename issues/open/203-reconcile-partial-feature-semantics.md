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

- [ ] Audit the listed done issues for placeholder or deferred semantics.
- [ ] Split any large remaining work into dedicated feature issues.
- [ ] Update `docs/language-reference/javascript-features.md` and related semantics docs with exact status.
- [ ] Add or update validation commands that prove semantic behavior, not only parsing/build success.
- [ ] Add a mechanical check or checklist rule if a done issue says "new issue needed" without an open follow-up.

Out of scope:

- [ ] Implementing the compiler/runtime behavior in this cleanup issue.

## Affected paths

Expected:

- `issues/open/`
- `issues/done/`
- `issues/index.md`
- `docs/language-reference/javascript-features.md`
- `docs/05-compatibility-and-semantics.md`
- `current-state.md`
- `scripts/check/`

Do not touch:

- `crates/`

## Acceptance criteria

- [ ] No done issue entry with placeholder/deferred semantics lacks a linked open follow-up or documented scope decision.
- [ ] Docs do not claim full implementation for placeholder behavior.
- [ ] `scripts/manager update-issue-index --check` passes.
- [ ] `scripts/manager check-issue-health` passes.
- [ ] `scripts/manager check-repo-smoke` passes.

## Validation

Required commands:

```sh
scripts/manager update-issue-index --check
scripts/manager check-issue-health
scripts/manager check-repo-smoke
```

Impacted commands:

```sh
rg -n "placeholder|deferred to follow-up|new issue needed" issues/done
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update affected language-reference / semantics docs

Current state:

- [ ] update `current-state.md` if current support status changes

Follow-up issues:

- [ ] split dedicated implementation issues as needed

## Completion evidence

Fill only when moving to `done/`.
