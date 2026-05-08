---
id: 5139
title: "Split APISample parser leftovers"
type: cleanup
area: frontend/syntax
class: design-ready
priority: P1
depends_on: []
blocks: [070]
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Split the remaining APISample parser leftovers that are not cleanly owned by the import/export, JSDoc, or watcher-arrow child issues.

Problem: issue 070 identified `APISample_linter.ts` and `APISample_transform.ts` as remaining parser/frontend leftovers, but they were still only described on the broad APISample parent.

## Problem

The APISample generated bucket mixes import/export module support, JSDoc parsing, watcher arrow-function lowering, transform API patterns, and a linter parser case. The parent is now closed as a superseded bucket, so the remaining parser leftovers need a narrow follow-up that can decide whether they belong under issue 543, issue 059, or a new implementation-ready parser issue.

## Current failure

Representative commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_linter.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_transform.ts
```

Issue 070 records the current classification:

```text
APISample_linter.ts: Parser: < token (multi-file test directive)
APISample_transform.ts: Parser / transform API
```

The APISample parent also records the broader representative parser diagnostic:

```text
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Failure: expected Semicolon, got Some(Ident("declare")) at 628..635
Context: declare var process: any;
```

## Desired final state

Each remaining APISample parser leftover is either assigned to an existing open issue with matching evidence or split into a new implementation-ready issue with exact reproduction, parser diagnostic, source context, and validation commands.

## Scope

In scope:

- [x] Run targeted triage for `APISample_linter.ts`.
- [x] Run targeted triage for `APISample_transform.ts`.
- [x] Decide whether each case belongs to issue 543, issue 059, or a new parser child.
- [x] Update duplicate references so issue 070 stays closed.

Out of scope:

- Implementing parser/runtime behavior.
- Broad APISample module/import-export support.
- JSDoc support and watcher arrow-function support.

## Affected paths

Expected:

- `issues/open/`
- `issues/open/070-implement-APISample.md`
- `reference/typescript/tests/cases/compiler/APISample_linter.ts`
- `reference/typescript/tests/cases/compiler/APISample_transform.ts`

Do not touch:

- `crates/`
- `docs/`

## Acceptance criteria

- [x] `APISample_linter.ts` has an exact triage result and assigned owner issue.
- [x] `APISample_transform.ts` has an exact triage result and assigned owner issue.
- [x] Any new child issue names the exact reference path, diagnostic/stdout change, and impacted commands.
- [x] Issue 070 remains closed as a superseded APISample parent.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_linter.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/APISample_transform.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] completed: `issues/open/5224-handle-package-json-virtual-sections-in-multifile-references.md`

## Resolution

Fresh triage showed both remaining APISample parser leftovers have the same
earliest blocker: the raw TypeScript reference file contains a virtual
`node_modules/typescript/package.json` section, and the compiler parses that
JSON body as TypeScript. Both files fail before their actual APISample source
section is reached.

`APISample_linter.ts`:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/APISample_linter.ts
result: UnsupportedSyntax: expected Semicolon, got Some(Colon) at 216..217
owner: issues/done/5224-handle-package-json-virtual-sections-in-multifile-references.md
```

`APISample_transform.ts`:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/APISample_transform.ts
result: UnsupportedSyntax: expected Semicolon, got Some(Colon) at 216..217
owner: issues/done/5224-handle-package-json-virtual-sections-in-multifile-references.md
```

This is not issue 543 yet because the import/export boundary is hidden behind
the earlier JSON virtual-section parser boundary. It is not the broad issue 059
because the failing construct is a concrete multi-file reference harness case.

## Completion evidence

Commits:

- this commit: issues: split APISample package-json virtual sections

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/APISample_linter.ts
result: pass for issue 5139; exact blocker recorded and assigned to issue 5224
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/APISample_transform.ts
result: pass for issue 5139; exact blocker recorded and assigned to issue 5224
date: 2026-05-06
```

Remaining risks:

- issue 5224 must still implement or classify JSON virtual-section handling.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

