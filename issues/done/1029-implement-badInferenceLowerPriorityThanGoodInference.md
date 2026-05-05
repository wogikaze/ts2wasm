---
id: 1029
title: "Implement Badinferencelowerprioritythangoodinference"
type: spike
area: frontend/semantics
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

Triage badInferenceLowerPriorityThanGoodInference across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `badInferenceLowerPriorityThanGoodInference` with diagnostics: type-system. Fresh smart triage shows the current blocker is a parser ASI gap after a multi-line `const` initializer, not the later type-inference behavior.

Problem: `badInferenceLowerPriorityThanGoodInference` is not a standalone implementation order; the executable parser slice is split to issue 5151.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by an implementation-ready child issue. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `python scripts/manager.py reference-triage ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts
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

- [x] created: `issues/open/5151-parse-asi-after-multiline-const-initializer.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: type system: badInferenceLowerPriorityThanGoodInference

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts
```

Source context:

```text
11 | const result = canYouInferThis(() => ({
12 |     a: { BLAH: 33 },
13 |     b: x => { }
14 | }))
15 | 
16 | result.BLAH;
```

Current compiler failure:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("result")) at 252..258
```

TypeScript oracle evidence:

```text
TypeScript diagnostics: none.
AST path: ExpressionStatement `result.BLAH;` after a semicolonless `const result = ...` declaration.
```

Resolution:

```text
The current blocker is parser ASI after a const declaration whose initializer ends before a newline expression statement. The later generic inference semantics remain outside this generated bucket split until parsing reaches them.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5151-parse-asi-after-multiline-const-initializer.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/badInferenceLowerPriorityThanGoodInference.ts
result: pass; reproduced parser semicolon expectation before `result.BLAH`
date: 2026-05-06
```

Remaining risks:

- none
