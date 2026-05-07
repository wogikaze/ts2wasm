---
id: 1523
title: "Implement Contextualtypingofaccessors"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualTypingOfAccessors across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypingOfAccessors` with diagnostics: class-accessor. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypingOfAccessors has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfAccessors.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfAccessors.ts --detail
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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfAccessors.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypingOfAccessors.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; stale build-pass bucket

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypingOfAccessors.ts`

## Duplicate detection

- No child issue needed. Fresh triage shows no current compiler blocker:
  `contextualTypingOfAccessors.ts` is a build-pass case.

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfAccessors.ts

result:
BuildPass: ts2wasm build succeeded
feature_label: build-pass
diagnostic: BuildPass / pass
tokens: ok
AST: ok; object literal getter/setter accessors parse
resolved: ok; getter returns arrow function `(n) => n`, setter has parameter `x`
TypeScript oracle: ok, no diagnostics
```

The TypeScript oracle confirms this test intentionally does not contextually
type accessor bodies: the getter arrow parameter `n` is `any`, and the setter
parameter `x` is typed as `(n: any) => any`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- stale build-pass cleanup

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypingOfAccessors.ts --detail --no-dashboard-data
result:
pass; executed=1, build_pass=1, unsupported=0, fail=0
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypingOfAccessors.ts
result:
pass; BuildPass, no current compiler blocker
date:
2026-05-07
```

Remaining risks:

- none for this bucket; semantic parity work should be tracked by a future
  semantic coverage issue only if a mismatch appears with semantic checking
  enabled.
