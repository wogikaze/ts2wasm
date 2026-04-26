# Issues

This directory contains executable work orders.

Issues are not design documents, progress logs, or long-term notes. They exist to move work from `open/` to `done/` with clear acceptance criteria and validation evidence.

## Project truth split

Use these locations consistently:

| Kind of information | Location |
|---|---|
| Final project design and intended contracts | `docs/` |
| Current implementation status and known gaps | `current-state.md` (repository root) |
| Future work | `issues/open/` |
| Completed work evidence | `issues/done/` |
| Reusable issue templates | `issues/templates/` |
| Agent FSM, task/verification state, run reports (machine contract) | `.agents/`, `reports/runs/` |

`done/` is historical evidence only. Do not treat completed issues as the source of current project truth.

## Mechanical checks

`scripts/manager check-issue-health` (run locally, in pre-commit, and in CI) fails if:

- the same `NNN-` id appears twice in `issues/open/` or twice in `issues/done/`, or the same `NNN` exists in both trees;
- a file `NNN-*.md` does not match the `**ID**:` (or yaml `id:`) value in the file;
- a file under `issues/done/` (except `*sample*` and `000-*`) still contains an unchecked list item (`- [ ]`);
- a `**Depends on**` id has no `issues/open/NNN-*.md` or `issues/done/NNN-*.md`;
- a backticked path under `crates/`, `docs/`, `fixtures/`, `scripts/`, `reference/`, `issues/`, `reports/`, `.github/`, `.agents/`, or `artifacts/` points to a path that does not exist (placeholders with `...` and similar are skipped);
- a JSON file under `.agents/state/` is not valid JSON (when `jq` is installed);
- `issues/index.md` fails `scripts/manager check-issue-index` (stale generated tables or an open id missing from Ready/Blocked).

`pre-commit` runs the generator so `issues/index.md` is refreshed and staged when needed, then runs this script.

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

## Sub-issues

For large features that require multiple phases (e.g., design then implementation), use sub-issue IDs:

- **Format**: `NNNa`, `NNNb`, `NNNc` (e.g., `017a`, `017b`)
- **Parent issue**: `NNN` (numeric only, e.g., `017`)
- **Filename**: `NNNa-short-title.md` (e.g., `017a-design-gc-strategy.md`)
- **Requirements**:
  - Parent issue must exist in `issues/open/` or `issues/done/`
  - No duplicate sub-issue IDs within the same parent (e.g., only one `017a`)
  - Sub-issues can depend on parent or other sub-issues
  - Validation scripts enforce parent existence and uniqueness

Use sub-issues when:
- A feature requires separate design and implementation phases
- A large refactor can be split into incremental steps
- Dependencies between phases are clear and linear

Do not use sub-issues for:
- Unrelated features (use separate numeric IDs instead)
- Minor follow-up tasks (create separate issues with new numeric IDs)

### 2. Work

During work:

- keep changes inside `Scope`
- avoid touching files listed under `Do not touch`
- update `current-state.md` / issues when behavior or status changes
- create follow-up issues instead of adding future TODOs to docs
- avoid recording progress logs inside final-state docs

### 3. Verify

Before completion:

- all acceptance criteria must be checked
- validation commands must be run or explicitly recorded under `Not run`
- `current-state.md` / issues sync must be resolved
- remaining risks must be written down

### 4. Complete

Move the issue from `open/` to `done/`.

Before moving:

- update `updated`
- fill `Completion evidence`
- set remaining risks to `none` or a concrete list
- run `scripts/manager update-issue-index` (do not hand-edit generated queue tables)

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

Use `current-state.md` at the repository root for current facts and `issues/open/` for future work.

## Completion quality bar

An issue is done only when:

- the desired final state is reached for the issue scope
- acceptance criteria are checked
- validation evidence is recorded
- `current-state.md` and issues are synchronized
- no hidden TODOs remain in final-state docs
- remaining risks are explicit

## Example Issues

### Good example

```markdown
---
id: 012
title: Fix computed property semantics bug
type: bug
area: runtime/semantics
class: implementation-ready
priority: P0
depends_on: none
---

## Summary

`obj["key"]` computed property access currently uses `$array_get` which performs an array tag check and returns `undefined` for non-array objects. This violates JavaScript semantics where computed property access should work on all objects.

## Desired final state

Computed property access works correctly on all objects, matching Node.js behavior.

## Scope

- Fix runtime property access logic
- Add differential test for computed property access
- Update current-state.md

## Affected paths

- `crates/cli/src/runtime/value.rs`
- `crates/cli/src/runtime/object.rs`
- `fixtures/core-semantics/computed-property.ts`
- `current-state.md`

## Acceptance criteria

- [ ] Computed property access returns correct value for object keys
- [ ] Differential test passes against Node.js
- [ ] No regression in array access

## Validation

```bash
cargo nextest run
# Differential test for computed property
node fixtures/core-semantics/computed-property.ts > expected.txt
iwasm fixtures/core-semantics/computed-property.wasm > actual.txt
diff expected.txt actual.txt
```

## Docs / current-state / issue sync

- Update current-state.md to reflect fixed semantics
- No follow-up issues needed

```

### Bad example

```markdown
---
id: 999
title: Fix various bugs
type: bug
area: runtime
class: implementation-ready
priority: P1
depends_on: none
---

## Summary

Fix some runtime bugs and improve performance.

## Desired final state

Runtime works better.

## Scope

- Fix bugs
- Optimize

## Acceptance criteria

- [ ] Fix bugs
- [ ] Make it faster
```

**Why this is bad**:
- Title is vague ("various bugs")
- Summary lacks specific problem description
- Scope is not bounded
- Acceptance criteria are not testable
- No validation commands
- No affected paths
- No docs sync plan
