---
id: 1170
title: "Implement Class"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: [5246]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage class across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `class` with diagnostics: unknown-unsupported. Fresh triage shows this is the same invalid `static` declaration inside a constructor body covered by issue 5246.

Problem: `class2.ts` is not a broad class implementation order. The first blocker is a focused invalid `static` statement diagnostic split to issue 5246.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/class2.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/class2.ts --detail
```

## Desired final state

This generated bucket is closed after splitting `issues/open/5246-report-static-declarations-inside-constructor-bodies.md`. Do not implement directly from this bucket.

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
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/class2.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/class2.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5246-report-static-declarations-inside-constructor-bodies.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/class2.ts`

## Duplicate detection

- `issues/done/067-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/done/106-implement-accessors.md` - Implement Accessors (same feature label, same group key, title overlap)
- `issues/done/193-implement-arguments.md` - Implement Arguments (same feature label, same group key, title overlap)
- `issues/done/201-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, same group key)
- `issues/done/225-implement-eval-annexb-function-declarations.md` - Implement eval and Annex B function declaration semantics (same feature label, same group key, title overlap)
- `issues/done/255-implement-private-class-element-runtime-semantics.md` - Implement private class element runtime semantics (same group key, title overlap)
- `issues/open/313-implement-array-builtin.md` - Implement array-builtin support (same feature label, same group key, title overlap)
- `issues/done/314-implement-string-builtin.md` - Implement string-builtin support (same feature label, same group key, title overlap)
- `issues/open/411-implement-annexb-ishtmldda.md` - Implement annexb-ishtmldda support (same feature label, same group key, title overlap)
- `issues/done/412-implement-arguments-object.md` - Implement arguments-object support (same feature label, same group key, title overlap)

## Smart triage

Fresh triage shows this generated class bucket is currently blocked by invalid
`static` syntax in constructor statement position.

### Smart triage: class2

- Issue class: `triage-needed`
- Feature label: `unknown-unsupported`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `unsupported expression: Some(SpannedToken { kind: Static, ... })`
- Path: `reference/typescript/tests/cases/compiler/class2.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/class2.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/class2.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
class foo { constructor() { static f = 3; } }
```

Compiler evidence:

```text
tokens: ok; `Static` token inside constructor body
ast: fails with generic unsupported expression
```

TypeScript oracle evidence:

```text
TS1128: Declaration or statement expected.
```

Split result:

- `issues/open/5246-report-static-declarations-inside-constructor-bodies.md`

## Completion evidence

Fill only when moving to `done/`.

The `class2` generated bucket is complete. The current failure is split to issue 5246.

Commits:

- split to `issues/open/5246-report-static-declarations-inside-constructor-bodies.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/class2.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax/unknown-unsupported
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/class2.ts
result: pass; AST construction reports unsupported expression at `static f = 3`, split to issue 5246
date: 2026-05-06
```

Remaining risks:

- none
