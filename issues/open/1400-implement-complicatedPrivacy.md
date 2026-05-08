---
id: 1400
title: "Implement Complicatedprivacy"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [432]
blocks: [5294]
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1400.

## Summary

Triage complicatedPrivacy across 1 failing reference test case and split this
generated bucket into implementation-ready child issue 5294.

## Problem

Reference test results originally showed 1 case failing in directory
`complicatedPrivacy` with diagnostics: import-export. Fresh smart triage shows
the current concrete blocker is name resolution for sibling namespace `m3` from
inside nested namespace `m1.m2`.

Problem: `complicatedPrivacy.ts` reports `UnresolvedName` for `m3` at
`implements m3.i3`; issue 5294 now owns the actionable fix.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedPrivacy.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complicatedPrivacy.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed because the actionable work has been split into
issue 5294. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

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
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complicatedPrivacy.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedPrivacy.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5294-resolve-sibling-namespaces-in-nested-namespace-scopes.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/complicatedPrivacy.ts`

## Duplicate detection

Fresh smart triage listed broad same-feature name-resolution candidates, but
none was an exact owner for sibling namespace lookup from inside a nested
namespace body. Issue 5287 is related but covers top-level same-file namespace
value access, not the unqualified sibling namespace `m3` in `namespace m1.m2`.

Resolution:

```text
Split to issue 5294: resolve sibling namespaces in nested namespace scopes.
```

## Smart triage

### Smart triage: Triage name resolution: complicatedPrivacy

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/complicatedPrivacy.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedPrivacy.ts
```

Failure location:

```text
message: unresolved name: `m3` at 226..228
line: 13, column: 4
```

Source context:

```ts
namespace m1 {
    export namespace m2 {
        export class C2 implements m3.i3 {
            public get p1(arg) {
                return new C1();
            }
        }
    }

    namespace m3 {
        export interface i3 {
            f55(): string;
        }
    }
}
```

Visible symbols before failure:

```text
function f1(c1: C1)
function f2(c2: C2)
class C2
```

Compiler evidence:

```text
tokens: ok; includes namespace m1, export namespace m2, class C2 implements m3.i3
ast: ok=False; UnresolvedName m3 at 226..228
resolved: ok=False; UnresolvedName m3 at 226..228
```

TypeScript oracle evidence:

```text
TS1054: A 'get' accessor cannot have parameters.
TS2322: Type 'null' is not assignable to type '(a: number) => C1'.
TS2693: 'number' only refers to a type, but is being used as a value here.
TS2564: Property 'x' has no initializer and is not definitely assigned in the constructor.
TS2694: Namespace 'mglo5' has no exported member 'i6'.
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
semantic_enabled=0
```

## Completion evidence

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedPrivacy.ts
result: pass; reproduces UnresolvedName m3 at 226..228
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complicatedPrivacy.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=0, unsupported=1, blocked=0
date: 2026-05-07
```

Remaining risks:

- After sibling namespace lookup advances, later diagnostics in this file are
  expected and may need separate issues.
