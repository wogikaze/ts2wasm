---
id: 1128
title: "Implement Chainedspecializationtoobjecttypeliteral"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5222]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage chainedSpecializationToObjectTypeLiteral across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `chainedSpecializationToObjectTypeLiteral` with diagnostics: object-literal. Fresh triage shows tokens and AST succeed; lowering stops at the current `issue-211` unknown receiver class diagnostic for `s.groupBy(...)`.

Problem: `chainedSpecializationToObjectTypeLiteral` is too broad for direct implementation. Its current observable blocker is now tracked by `issues/open/5222-support-interface-typed-method-calls-on-erased-locals.md`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts --detail
```

## Desired final state

This generated bucket is superseded by `issues/open/5222-support-interface-typed-method-calls-on-erased-locals.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable blocker into issue 5222
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this issue and issue 5222

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
- [x] Issue 5222 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Issue 5222 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Issue 5222 acceptance names the exact fixture/reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts
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

- [x] created: `issues/open/5222-support-interface-typed-method-calls-on-erased-locals.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts`

## Duplicate detection

- `issues/open/5201-parse-object-type-literal-call-signatures.md` is not an exact match: 1128's AST succeeds, while 5201 is parser-owned and fails on object type literal call-signature annotation parsing.
- `issues/open/5195-support-callable-interface-typed-local-calls.md` is related but not an exact match: it handles direct calls to callable interface locals, while 1128 calls a method through an interface-typed receiver.
- Broad object-literal buckets are not exact matches because the first blocker is a method-call receiver classification diagnostic.

## Smart triage

### Smart triage: Triage object literal: chainedSpecializationToObjectTypeLiteral

- Issue class: `triage-needed`
- Feature label: `object-literal`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts --detail --no-dashboard-data
```

Source context:

```text
interface Sequence<T> {
    each(iterator: (value: T) => void): void;
    map<U>(iterator: (value: T) => U): Sequence<U>;
    filter(iterator: (value: T) => boolean): Sequence<T>;
    groupBy<K>(keySelector: (value: T) => K): Sequence<{ key: K; items: T[]; }>;
}

var s: Sequence<string>;
var s2 = s.groupBy(s => s.length);
var s3 = s2.each(x => { x.key /* Type is K, should be number */ });
```

Current compiler failure:

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `groupBy` at 326..350
```

Compiler evidence:

- Tokens succeed for the interface declaration, method signatures, generic parameters, object type literal return, and method calls.
- AST succeeds with `Let s = Undefined`, `Let s2 = Call(Member(Ident s, "groupBy"), ...)`, and `Let s3 = Call(Member(Ident s2, "each"), ...)`.
- Resolved/lowered output stops in lower_program at `s.groupBy(...)`.

TypeScript oracle evidence:

```text
TS2454: Variable 's' is used before being assigned.
```

TypeScript hints include:

```text
s: Sequence<string>
s2: Sequence<{ key: number; items: string[]; }>
x: { key: number; items: string[]; }
```

Resolution:

```text
The current blocker is now tracked by child issue 5222. It is narrower than the generated bucket: support or precisely diagnose method calls on erased locals whose TypeScript annotation is an interface type.
```

## Completion evidence

Fill only when moving to `done/`.

chainedSpecializationToObjectTypeLiteral triage is complete. The actionable
blocker is tracked by child issue 5222.

Commits:

- child issue: `issues/open/5222-support-interface-typed-method-calls-on-erased-locals.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax object-literal
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/chainedSpecializationToObjectTypeLiteral.ts
result: pass; reproduced issue-211 unknown receiver class for method groupBy and split to issue 5222
date: 2026-05-06
```

Remaining risks:

- none
