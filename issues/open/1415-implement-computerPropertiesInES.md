---
id: 1415
title: "Implement Computerpropertiesines"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [059]
blocks: [5299]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1415.

## Summary

Triage computerPropertiesInES across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case fails in directory `computerPropertiesInES`. Fresh triage on 2026-05-07 shows the parser accepts the template-literal computed binding key, and the real blocker is issue-251 runtime-subset lowering for a computed object binding alias inside an arrow parameter.

Problem: computerPropertiesInES has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue 5299 contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to issue 5299

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts`

## Duplicate detection

- Issue 5297 is related but not a duplicate: it owns declaration binding aliases and explicitly excludes parameter binding patterns.
- Issue 251 is done and provides the current source-spanned unsupported boundary.

## Smart triage

Generated on 2026-05-07:

```text
Feature label: runtime-subset
Diagnostic: UnsupportedRuntimeSubset / unsupported-feature-boundary
Path: reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts
Failure: issue-251 object binding aliases must use identifier keys in this runtime slice at 54..87
line 3, column 13
Source context:
1 | // @strict: false
2 | // @target: es5, es2015
3 | const b = ({ [`key`]: renamed }) => renamed;
Visible symbols before failure:
- binding b
```

Compiler evidence:

```text
tokens: ok; LeftParen LeftBrace LeftBracket TemplateLiteral("key") RightBracket Colon Ident("renamed")
ast: ok; ArrowFn param "{[String { value: \"key\" }]: renamed}"
resolved: UnsupportedRuntimeSubset issue-251 object binding aliases must use identifier keys
TypeScript oracle: ok, diagnostics=[]
```

Coverage result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computerPropertiesInES5ShouldBeTransformed.ts --detail --no-dashboard-data
result: executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-07
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5299-lower-computed-object-binding-parameters.md`.

Validation result:

```text
command: python scripts/manager.py update-issue-index && python scripts/manager.py update-issue-index --check && python scripts/manager.py check-issue-health && python scripts/manager.py check-issue-readiness -- --fail-ready-below 80 && git diff --check
result: pass
date: 2026-05-07
```

Remaining risks:

- Child issue 5299 still needs implementation.
