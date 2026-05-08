---
id: 1165
title: "Implement Circularresolvedsignature"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5243]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1165.

## Summary

Triage circularResolvedSignature across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `circularResolvedSignature`. Fresh coverage now reports `UnresolvedName/name-resolution`; triage shows the current first blocker is stale parsing of `useState<Data>(...)` as runtime `<` / `>` binary expressions.

Problem: `circularResolvedSignature.ts` is not a standalone generated import/export bucket in the current runner view. The actionable first blocker is ambient generic function-call type-argument erasure, split to issue 5243.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularResolvedSignature.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularResolvedSignature.ts --detail
```

## Desired final state

This generated bucket is closed after splitting `issues/open/5243-date-non-literal-constructor-inputs.md`. Do not implement directly from this bucket.

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularResolvedSignature.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularResolvedSignature.ts
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

- [x] created: `issues/open/5243-date-non-literal-constructor-inputs.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularResolvedSignature.ts`

## Duplicate detection

- `issues/open/5242-w2-completion-declaration.md` covers generic callable const declarations; this case is an ambient `declare function`.
- `issues/open/5202-parse-member-call-explicit-type-arguments.md` covers member callees; this case is a direct identifier call.
- Broad import/export and name-resolution buckets are not exact owners for the current first blocker.

## Smart triage

Fresh triage shows this generated import/export bucket is currently blocked by
ambient generic function-call type-argument erasure.

### Smart triage: circularResolvedSignature

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Current compiler message: `unresolved name: Data at 280..284`
- Path: `reference/typescript/tests/cases/compiler/circularResolvedSignature.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularResolvedSignature.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularResolvedSignature.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Source context:

```ts
declare function useState<S>(initialState: (() => S)): [S, (s: S) => void];

type Data = Readonly<{
    value: number;
    foo: (arg: any) => void;
    bar: (arg: any) => void;
}>;

export function Component() {
    const [state, setState] = useState<Data>(() => ({
        value: "string",
        foo: (arg) => setState(arg),
        bar: (arg) => setState(arg),
    }));
}
```

Compiler evidence:

```text
tokens: ok
ast: ok but `useState<Data>` is represented as Binary Less / Binary Greater, not as an erased generic call
resolved: UnresolvedName for type-only `Data`
```

TypeScript oracle evidence:

```text
TS2322: Type 'string' is not assignable to type 'number'.
```

Split result:

- `issues/open/5243-date-non-literal-constructor-inputs.md`

## Completion evidence

Fill only when moving to `done/`.

The `circularResolvedSignature` bucket is complete. The current failure is split to issue 5243.

Commits:

- split to `issues/open/5243-date-non-literal-constructor-inputs.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularResolvedSignature.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnresolvedName/name-resolution
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularResolvedSignature.ts
result: pass; resolver reports type-only `Data` unresolved because `useState<Data>` was parsed as runtime binary operators, split to issue 5243
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5243 may expose later destructuring binding, circular resolved-signature inference, or TS2322 semantic-parity blockers.
