---
id: 1240
title: "Implement Classstaticpropertytypeguard"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1240.

## Summary

Triage `classStaticPropertyTypeGuard` across 1 failing reference test case and
close it as superseded by the existing typed modified static class field parser
issue.

## Problem

Reference test results show 1 case failing in directory
`classStaticPropertyTypeGuard` with diagnostics: parser-syntax. Fresh triage
confirms the exact blocker is parsing `private static _a: string | undefined;`
as a method declaration after `private static`.

Problem: `classStaticPropertyTypeGuard.ts` reports `expected LeftParen, got Some(Ident("_a"))` at the typed modified static class field. Existing issue 5288 already owns this parser shape.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticPropertyTypeGuard.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classStaticPropertyTypeGuard.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5288 covers the typed modified static class field parser blocker
- [x] Confirm related issue 5271 covers the untyped modified static field subset
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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
- [x] Superseding issue 5288 contains an exact parser/frontend work order
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classStaticPropertyTypeGuard.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classStaticPropertyTypeGuard.ts
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

- [x] superseded by `issues/done/5288-parse-typed-modified-static-class-fields.md`
- [x] related subset: `issues/open/5271-parse-modified-static-class-fields.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classStaticPropertyTypeGuard.ts`

## Duplicate detection

- `issues/done/5288-parse-typed-modified-static-class-fields.md` - exact owner for `public/private static name: Type` class fields after accessibility modifiers
- `issues/open/5271-parse-modified-static-class-fields.md` - related untyped `private static x = 10` / `public static y = 10` subset
- Broad parser-syntax buckets are no-match for this current evidence because 5288 is the narrower implementation-ready owner.

## Smart triage

Fresh triage shows this generated parser-syntax bucket is a duplicate of the
existing typed modified static class field parser issue.

### Smart triage: classStaticPropertyTypeGuard

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `expected LeftParen, got Some(Ident("_a"))`
- Path: `reference/typescript/tests/cases/compiler/classStaticPropertyTypeGuard.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticPropertyTypeGuard.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classStaticPropertyTypeGuard.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
```

Source context:

```ts
class A {
    private static _a: string | undefined;

    public get a(): string {
        if (A._a) {
            return A._a;
        }
        return A._a = 'helloworld';
    }
}
```

Compiler evidence:

```text
tokens: ok; private, static, Ident("_a"), Colon, Ident("string"), Pipe, Undefined, Semicolon are present
ast: fails with UnsupportedSyntax `expected LeftParen, got Some(Ident("_a"))`
resolved: same parser failure
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
AST path: ClassDeclaration -> PropertyDeclaration -> Identifier `_a`
```

Superseding owner:

- `issues/done/5288-parse-typed-modified-static-class-fields.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Superseded by existing issue 5288; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classStaticPropertyTypeGuard.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; current typed modified static field parser blocker superseded by 5288
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classStaticPropertyTypeGuard.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; executed=1, unsupported=1
date: 2026-05-07
```

Remaining risks:

- After issue 5288 lands, the representative may expose static field runtime
  lowering or strict-null type-guard semantic parity gaps.
