---
id: 1409
title: "Implement Computedpropertiesnarrowed"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: [5228]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1409.

## Summary

Closed as superseded by
`issues/open/5228-w0-wasm-binary-backend-mvp.md`.

Fresh triage shows the current first blocker is the simple computed object
literal key parser gap for `{ [x]: 1 }`.

## Problem

Reference test results originally showed 1 case failing in directory
`computedPropertiesNarrowed` with diagnostics: import-export. Fresh focused
coverage now reports `UnsupportedSyntax` at the first object literal computed
key.

Problem: 1409 is not a standalone computed-property narrowing work order in the
current runner view. The first actionable blocker is the parser behavior
already owned by issue 5228.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesNarrowed.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesNarrowed.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implementation should proceed through issue
5228 until object literals accept simple identifier computed keys.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5228
- [x] Preserve exact reproduction commands and representative evidence

Out of scope:

- Direct implementation from this generated bucket
- TypeScript narrowing diagnostics after object literal computed keys parse

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
- [x] Existing issue 5228 owns the current simple computed object literal key parser blocker
- [x] This issue includes failing path, diagnostic code, source context, compiler evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesNarrowed.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesNarrowed.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only
- `cargo nextest run`; issue metadata only

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by: `issues/open/5228-w0-wasm-binary-backend-mvp.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/computedPropertiesNarrowed.ts`

## Duplicate detection

- `issues/open/5228-w0-wasm-binary-backend-mvp.md` owns the
  current first blocker: parsing an object literal property assignment whose
  computed key is a simple identifier such as `{ [x]: 1 }`.
- `issues/open/5209-support-class-instance-method-receiver-calls.md`
  covers later non-simple key expressions such as binary expressions.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage parser syntax: computedPropertiesNarrowed

- Issue class: triage-needed
- Feature label: parser-syntax
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/computedPropertiesNarrowed.ts
```

Source context:

```ts
const x: 0 | 1 = Math.random()? 0: 1;
declare function assert(n: number): asserts n is 1;
assert(x);
export let o = {
    [x]: 1
}
```

Compiler evidence:

```text
tokens: ok; includes LeftBracket Ident("x") RightBracket Colon Number(1)
ast: UnsupportedSyntax expected Dot, got Some(RightBracket) at 205..206
resolved: same parser diagnostic
```

TypeScript oracle evidence:

```text
ok=true, diagnostics=[]
binding o has type { 1: number; }
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

## Completion evidence

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/computedPropertiesNarrowed.ts
result: pass; reproduces expected Dot / RightBracket parser failure for `[x]`
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/computedPropertiesNarrowed.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, blocked=0
date: 2026-05-07
```

Remaining risks:

- After issue 5228 advances simple computed object literal keys, this reference
  file is expected to expose export/module handling, `Symbol()` support, and
  TypeScript computed-property narrowing diagnostics.
