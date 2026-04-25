# Issues

This directory contains executable work orders.

Issues are not design documents, progress logs, or long-term notes. They exist to move work from `open/` to `done/` with clear acceptance criteria and validation evidence.

## Project truth split

Use these locations consistently:

| Kind of information | Location |
|---|---|
| Final project design and intended contracts | `docs/` |
| Current implementation status and known gaps | `docs/current-state.md` |
| Future work | `issues/open/` |
| Completed work evidence | `issues/done/` |
| Reusable issue templates | `issues/templates/` |

`done/` is historical evidence only. Do not treat completed issues as the source of current project truth.

## Directory layout

```text
issues/
  README.md
  index.md
  open/
    001-example.md
  done/
    000-completed-example.md
  templates/
    issue.md
```

## How to read an issue

Read an issue in this order:

1. `Summary`
2. `Desired final state`
3. `Scope`
4. `Affected paths`
5. `Acceptance criteria`
6. `Validation`
7. `Docs / current-state / issue sync`

Do not start implementation from `Notes`. `Notes` is only a small hint section.

## Issue lifecycle

### 1. Create

Create a small work order under `issues/open/`.

Recommended filename:

```text
NNN-short-kebab-title.md
```

Examples:

```text
001-define-runtime-value-representation.md
002-add-capability-manifest-output.md
003-clean-docs-current-state-split.md
```

One issue should be small enough to hand to an agent.

Large work should be split into multiple issues rather than kept as one giant checklist.

### 2. Work

During work:

- keep changes inside `Scope`
- avoid touching files listed under `Do not touch`
- update docs/current-state/issues when behavior or status changes
- create follow-up issues instead of adding future TODOs to docs
- avoid recording progress logs inside final-state docs

### 3. Verify

Before completion:

- all acceptance criteria must be checked
- validation commands must be run or explicitly recorded under `Not run`
- docs/current-state/issues sync must be resolved
- remaining risks must be written down

### 4. Complete

Move the issue from `open/` to `done/`.

Before moving:

- update `updated`
- fill `Completion evidence`
- set remaining risks to `none` or a concrete list
- update `issues/index.md`

## Issue classes

| Class | Meaning |
|---|---|
| `design-ready` | Needs a design decision or contract before implementation |
| `implementation-ready` | Ready for code changes |
| `verification-ready` | Code exists; needs tests, review, or gate verification |
| `docs-ready` | Documentation cleanup or contract update |
| `blocked` | Cannot proceed until a blocker is resolved |

## Issue types

| Type | Meaning |
|---|---|
| `feature` | Adds user-visible or project-visible behavior |
| `bug` | Fixes incorrect behavior |
| `refactor` | Changes structure without intended behavior change |
| `docs` | Changes documentation only |
| `test` | Adds or fixes tests/fixtures without product behavior changes |
| `infra` | Changes CI, scripts, gates, or repo infrastructure |
| `cleanup` | Removes stale, duplicated, or misleading material |
| `spike` | Investigation with a required decision output |

## Docs rule

Normal docs describe the final intended project state.

Do not put these into normal docs:

- TODO lists
- stale history
- progress logs
- temporary implementation notes
- future milestone promises
- current implementation limitations

Use `docs/current-state.md` for current facts and `issues/open/` for future work.

## Completion quality bar

An issue is done only when:

- the desired final state is reached for the issue scope
- acceptance criteria are checked
- validation evidence is recorded
- docs/current-state/issues are synchronized
- no hidden TODOs remain in final-state docs
- remaining risks are explicit
