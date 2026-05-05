---
id: 1013
title: "Implement Avoid"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage avoid across 1 failing reference test case and split this generated bucket into a smaller implementation-ready child issue.

## Problem

Reference test results show 1 case fails in directory `avoid` with diagnostics: method-call. Fresh triage shows the specific blocker is an issue-211 method call whose receiver is a `new` expression.

Problem: avoid has 1 reference failure that is now tracked by child issue 5142.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoid.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/avoid.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/done/5142-support-class-method-call-on-new-expression-receiver.md`.

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
- [x] Child issue 5142 contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/avoid.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoid.ts
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

- [x] created: `issues/done/5142-support-class-method-call-on-new-expression-receiver.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/avoid.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage method call: avoid

- Issue class: `triage-needed`
- Feature label: `method-call`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/avoid.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoid.ts
```

Failure location:

```json
{
  "code": "UnsupportedSyntax",
  "message": "issue-211: method `g` requires an identifier receiver at 228..239",
  "span_start": 228,
  "span_end": 239,
  "line": 18,
  "column": 24,
  "feature_label": "method-call",
  "error_type": "parser-or-frontend-unsupported"
}
```

Source context:

```text
15 |     }
16 | }
17 |
18 | var z=new C().g(); // error void fn
19 | var N=new f();  // ok with void fn
20 |
```

Visible symbols before failure include local function `f`, local class `C`, and binding `z` initialized as `new C().g()`.

Compiler evidence:

```text
AST: Let z = Call(Member(New(Ident("C"), args=[]), property="g"), args=[])
resolved: lower_program reports issue-211: method `g` requires an identifier receiver
TypeScript oracle: ok, no diagnostics; binding `z` has type `void`
```

## Completion evidence

Commits:

- pending local commit

Validation result:

```text
command:
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoid.ts
result:
emitted UnsupportedSyntax / method-call report for `new C().g()`; split to issue 5142
date:
2026-05-06
```

Remaining risks:

- none
