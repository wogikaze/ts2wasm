---
name: docs-workflow
description: Use when adding/editing docs under docs/. Enforces state separation between final-state docs, current-state, issues, and historical state.
---

# Docs workflow

Use this skill only for documentation changes under docs/ and README-level documentation changes.

## Table of Contents

- [Mise: run before you finish](#mise-run-before-you-finish-required-if-you-touched-paths-under-verification)
- [Core Policy](#core-policy)
- [State Separation Rules](#state-separation-rules)
  - [Final-state docs](#final-state-docs)
  - [current-state.md](#current-state-md)
  - [Issues](#issues)
  - [Historical state](#historical-state)
- [Document Type Rules](#document-type-rules)
- [README Rules](#readme-rules)
- [Cleanup Procedure](#cleanup-procedure)
- [Required Searches](#required-searches)
- [Validation](#validation)
- [Common Traps](#common-traps)
- [Output Checklist](#output-checklist)
- [Related Skills](#related-skills)

## Mise: auto-execute after making changes (required if you touched paths under verification)

**Automatically run and pass the commands that match your change after making changes.** Without `mise`, use `mise` with the same subcommand. First time: `mise trust` ([docs](https://mise.jdx.dev/cli/trust.html)).

- Always: `mise run fmt` (touches `*.md` / Rust in examples)
- Default smoke: `mise run check`
- If docs claim test or compiler behavior: `mise run nextest` (and `mise run clippy` if Rust examples changed)
- If you edited `issues/` cross-links or agent/issue docs: `mise run check issues`
- **Auto-commit changes after verification passes** (commit message based on change description)

This skill enforces a strict separation between:

1. final-state project documentation
2. current implementation state
3. future work items
4. historical/dead state

Docs are not a dumping ground for progress logs, milestone leftovers, stale plans, or half-implemented TODOs.

## Core Policy

Project docs describe the intended final shape of ts2wasm.

A normal docs page should read as the target design contract of the project, not as a diary of what currently works.

Allowed in normal docs:

- project definition
- final architecture
- semantic contracts
- runtime ABI contracts
- capability model
- test status schema
- compatibility policy
- coding standard
- accepted design rules
- stable command contracts
- expected final behavior

Not allowed in normal docs:

- old implementation state
- “currently this does not work”
- “M6 will implement this”
- “temporary workaround”
- unresolved TODO list
- progress logs
- stale milestone plans
- abandoned alternatives
- copied issue text
- mixed past/current/future wording

## State Separation Rules

### Final-state docs

Write final-state docs as normative project contracts.

Use present tense where possible.

Good:

- `The compiler lowers TypeScript expressions into JS-value IR.`
- `The runtime represents string values as managed heap objects.`
- `The WASI target imports only the capabilities declared in the manifest.`

Bad:

- `The compiler will lower TypeScript expressions into JS-value IR.`
- `Currently strings are still handled by the old evaluator.`
- `TODO: add capability manifest support.`
- `In M6 we plan to introduce stdin support.`

Future-looking target design is allowed only when written as the accepted intended architecture, not as a task.

### current-state.md

Actual implementation status belongs in:

- `current-state.md` (repository root)

Use this file for:

- what is implemented now
- what is partially implemented now
- what is blocked now
- known gaps between final-state docs and implementation
- current command support
- current coverage numbers
- current runner limitations
- current unsupported features

Rules for `current-state.md`:

1. It must be factual.
2. It must be easy to delete or rewrite.
3. It must not define project goals.
4. It must not contain long-term design rationale.
5. It must link to issues for unfinished work.
6. It must not duplicate large sections from final-state docs.
7. It must be updated when implementation catches up or diverges.

Recommended structure:

```md
# Current state

Last audited: YYYY-MM-DD

## Summary

## Implemented

## Partially implemented

## Not implemented

## Known divergence from final-state docs

## Open issues

## Validation commands
```

### Issues

Future work belongs in `issues/`.

Use issues for:

- TODOs
- milestones
- implementation slices
- missing feature work
- known bugs
- cleanup work
- blocked work
- deferred design decisions
- acceptance criteria
- validation commands

A docs change that finds future work must either:

1. move it into an existing issue, or
2. create a new issue file, or
3. explicitly state why no issue was created.

Normal docs must not keep TODOs after the issue is created.

### Historical state

Past state should usually be deleted.

Do not preserve old implementation descriptions “for context” in normal docs.

Allowed historical material:

- ADRs that record accepted or rejected design decisions
- migration notes that are still needed by users
- changelog entries if the project has a changelog
- issue discussion in issue files

Rules:

1. Historical state must not appear as if it is current.
2. Historical state must not remain in architecture/reference docs.
3. Rejected designs belong in ADR/issue context, not in final-state docs.
4. If the old state no longer affects decisions, delete it.

## Document Type Rules

Classify each docs page before editing.

Allowed document types:

1. Project definition
2. Architecture
3. Semantic contract
4. Runtime ABI / capability contract
5. Testing and coverage contract
6. Coding standard
7. Current state
8. ADR / decision record
9. User-facing command reference
10. Contributor workflow

Each docs page should have one primary type.

Do not mix:

- architecture + TODO list
- reference + progress log
- current state + final design
- roadmap + implementation notes
- coding standard + issue backlog

## README Rules

README should be a project entrypoint.

README may contain:

- one-paragraph project definition
- build/test quickstart
- docs index
- current stability warning
- link to `current-state.md` (repository root)
- link to issues

README must not contain:

- long roadmap
- stale milestone table
- detailed architecture
- feature TODO list
- old benchmark claims
- duplicated current-state details

## Cleanup Procedure

When editing docs, run this audit mentally and mechanically.

### 1. Identify temporal category

For each changed paragraph, classify it as:

- final-state contract
- current-state fact
- future task
- historical note
- rationale

Then move it:

| Category             | Destination                      |
| -------------------- | -------------------------------- |
| final-state contract | appropriate docs page            |
| current-state fact   | `current-state.md` (repo root)   |
| future task          | `issues/`                        |
| historical note      | delete, ADR, changelog, or issue |
| rationale            | explanation doc or ADR           |

### 2. Delete stale text

Delete text that only explains old behavior.

Do not soften stale text with “previously”.
Do not leave stale text under “Historical note” unless it is still operationally useful.

### 3. Move TODOs to issues

Any TODO-like text in docs must be converted into an issue.

Search patterns:

- `TODO`
- `FIXME`
- `not yet`
- `currently`
- `temporary`
- `workaround`
- `future`
- `planned`
- `will`
- `later`
- `M[0-9]`
- `phase`
- `roadmap`
- `blocked`
- `unsupported`

Issue file should include:

```md
# Title

## Problem

## Desired final state

## Implementation notes

## Acceptance criteria

## Validation commands

## Docs to update when done
```

After issue creation, replace the docs TODO with either:

- final-state text, or
- a short current-state entry in `current-state.md` (repo root)

Do not leave both.

### 4. Sync current-state

If the docs describe final behavior that is not implemented yet, add or update `current-state.md` (repo root).

The current-state entry should name the divergence and link to the issue.

Example:

```md
## Known divergence from final-state docs

- Capability manifest emission is specified in `docs/03-api-and-host-capability.md`, but the CLI does not yet emit the manifest by default.
  Tracking: `issues/WB-123.md`
```

### 5. Preserve output contracts

When docs describe script output, TestRecord JSONL, CLI output, or coverage tables, verify that the docs match actual code/tests.

Do not update contract docs based on intention alone unless current divergence is recorded in `current-state.md` (repo root) and an issue exists.

## Required Searches

Before finalizing a docs cleanup, run targeted searches.

Recommended commands:

```sh
rg -n "TODO|FIXME|not yet|currently|temporary|workaround|future|planned|later|roadmap|blocked|unsupported" docs README.md
rg -n "M[0-9]+|phase|milestone" docs README.md
rg -n "will|should eventually|in the future" docs README.md
rg -n "current-state|Current state|Known divergence" docs README.md issues
```

For code contract docs, also search the code/tests:

```sh
rg -n "TestRecord|suite|status|unsupported|blocked|skip-with-reason" crates scripts docs
rg -n "capability|manifest|RuntimeLinkPlan|fd_read|fd_write" crates scripts docs
rg -n "fixtures/" crates scripts docs issues
```

Use `ast-grep` when checking structural code contracts.

Examples:

```sh
ast-grep --lang rust -p 'struct $NAME { $$$ }' crates
ast-grep --lang rust -p 'enum $NAME { $$$ }' crates
```

## Validation

Always run:

```sh
cargo fmt --all --check
```

For docs-only changes, run at least:

```sh
rg -n "TODO|FIXME|not yet|currently|temporary|workaround|future|planned|later|roadmap|blocked|unsupported" docs README.md
```

When docs mention scripts or generated reports, also run:

```sh
scripts/check/shell-syntax.sh
```

When docs mention test status schema, coverage, fixture behavior, runtime ABI, or CLI contracts, run impacted tests:

```sh
cargo nextest run
```

A smaller impacted test set is acceptable only if the final report names exactly what was not run.

## Common Traps

- README becomes a second roadmap.
- Architecture docs describe current partial implementation.
- `current-state.md` becomes a permanent junk drawer.
- TODO remains in docs after issue creation.
- Old milestone tables remain because they “might be useful”.
- A final-state doc says “will” when it should state the contract directly.
- Current limitation is written into the architecture as if it is design.
- Issue text is copied into docs instead of linked.
- Docs claim coverage support that scripts/tests do not enforce.
- A stale fixture path remains in docs after fixture migration.
- A stale script command remains in docs after script option changes.
- Historical rejected design remains next to accepted design.
- `unsupported` and `blocked` are described as passing coverage.
- Current-state facts are updated without a date or validation command.
- Docs cleanup changes project policy without updating coding standard or issues.

## Related Skills

- fixtures-workflow: for fixture path updates when docs reference fixtures
- scripts-workflow: for script command updates when docs reference scripts
- issues-workflow: for moving TODOs from docs to issues
- checklist-to-issue: for converting checklist items to issues

## Output Checklist

Final response must include:

1. Docs files changed
2. Deleted stale/historical sections
3. Current-state entries added or updated
4. Issues created or updated for future work
5. Final-state docs updated
6. README changes, or `none`
7. Contract changes, or `none`
8. Commands run
9. Remaining stale-risk areas
10. Follow-up issues that now own the remaining work
