---
id: 1124
title: "Implement Cf"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5220]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage cf across 1 failing reference test case and fold the current blocker into an implementation-ready issue.

## Problem

Reference test results show 1 case failing in directory `cf` with diagnostics: name-resolution. Fresh triage shows the specific blocker is `UnresolvedName` for `k` after `var k` was declared in a previous `for` initializer.

Problem: cf has 1 reference failure whose actionable blocker is now tracked by `issues/done/5220-preserve-ambient-function-parameters-for-arity.md`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cf.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cf.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/done/5220-preserve-ambient-function-parameters-for-arity.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable blocker into issue 5220
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this issue and issue 5220

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
- [x] Issue 5220 contains an exact `mise run reference-triage -- ...` command
- [x] Issue 5220 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5220 acceptance names the exact reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cf.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cf.ts
```

Not run:

- `cargo fmt --all --check`; issue triage only, no Rust code changed
- `cargo nextest run`; issue triage only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5220-preserve-ambient-function-parameters-for-arity.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cf.ts`

## Duplicate detection

- `issues/done/427-implement-duplicate-local.md` - Implement duplicate-local support (same feature label, same group key, title overlap)

## Smart triage

Generated on 2026-05-06:

- command: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cf.ts`
- issue class: `triage-needed`
- feature label: `name-resolution`
- diagnostic: `UnresolvedName` / `resolver-symbol`
- message: `unresolved name: k at 0..0`
- child issue: `issues/done/5220-preserve-ambient-function-parameters-for-arity.md`

Representative source:

```text
for (var k=0;k<10;k++) {
    z;
    break;
}
for (k=0;k<10;k++) {
    if (k==6) {
        continue;
    }
    break;
}
```

Compiler evidence:

```text
tokens: ok
AST: function f includes first For with init Let k and second For with init assignment/read of k
resolved: UnresolvedName for k during resolve_names
TypeScript oracle: ok, diagnostics: []; binding k has type number at line 48
```

## Completion evidence

Closed as a generated triage bucket. The actionable loop `var` hoisting/name
resolution blocker is tracked by child issue 5220.

Commits:

- this fold commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cf.ts
result: fail with UnresolvedName for sibling-loop `k`; split to issue 5220
date: 2026-05-06
```

Remaining risks:

- After issue 5206 is implemented, `cf.ts` may expose later control-flow or unreachable-code diagnostics.
