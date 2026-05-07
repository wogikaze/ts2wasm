---
id: 1168
title: "Implement Circularlyconstrainedmappedtypecontainingconditionalnoinfiniteinstantiationdepth"
type: spike
area: frontend/semantics
class: done
priority: P1
depends_on: [5245]
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case failing in directory `circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth` with diagnostics: type-system. Fresh triage shows the current first blocker is parser support for an interface construct signature, before mapped/conditional type semantics.

Problem: this generated type-system bucket is not directly implementation-ready in the current runner view. The first actionable blocker is split to issue 5245.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts --detail
```

## Desired final state

This generated bucket is closed after splitting `issues/done/5245-iterator-protocol-runtime.md`. Do not implement directly from this bucket.

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts
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

- [x] created: `issues/done/5245-iterator-protocol-runtime.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts`

## Duplicate detection

- `issues/done/5201-parse-object-type-literal-call-signatures.md` covers object type literal call signatures, not interface construct signatures.
- Broad type-system buckets are not exact owners for the current parser diagnostic.

## Smart triage

Fresh triage shows this generated type-system bucket is currently blocked by
interface construct-signature parsing.

### Smart triage: circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth

- Issue class: `triage-needed`
- Feature label: `type-system`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current compiler message: `unsupported expression: Some(SpannedToken { kind: Greater, ... }) at 249..250`
- Path: `reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=type-system:1
```

Source context:

```ts
interface ComponentClass<P = {}> {
    new (props: P, context?: any): Component<P>;
    propTypes?: WeakValidationMap<P>;
    defaultProps?: Partial<P>;
    displayName?: string;
}
```

Compiler evidence:

```text
tokens: ok
ast: fails at construct signature return type `Component<P>`
resolved: same UnsupportedSyntax because AST construction failed
```

TypeScript oracle evidence:

```text
TypeScript accepts the construct signature and reaches later TS2344 mapped/conditional type diagnostics near line 65.
```

Split result:

- `issues/done/5245-iterator-protocol-runtime.md`

## Completion evidence

Fill only when moving to `done/`.

The `circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth` bucket is complete. The current failure is split to issue 5245.

Commits:

- split to `issues/done/5245-iterator-protocol-runtime.md`

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts --detail --no-dashboard-data
result: pass on the main checkout; 1 executed, current failure is UnsupportedSyntax/type-system
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/circularlyConstrainedMappedTypeContainingConditionalNoInfiniteInstantiationDepth.ts
result: pass; AST construction reports unsupported expression at interface construct signature, split to issue 5245
date: 2026-05-06
```

Remaining risks:

- Implementing issue 5245 may expose mapped type, conditional type, or circular instantiation-depth semantic blockers later in this reference file.
