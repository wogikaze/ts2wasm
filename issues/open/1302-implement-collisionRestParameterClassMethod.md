---
id: 1302
title: "Implement Collisionrestparameterclassmethod"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1302.

## Summary

Triage collisionRestParameterClassMethod across 1 reference case and close it
as superseded by the existing class method overload-signature issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionRestParameterClassMethod` with diagnostics: parser-syntax. Fresh
triage shows tokens and AST now succeed; the current first blocker is
`DuplicateFunction` for bodyless class method overload signatures with rest
parameters.

Problem: valid TypeScript class method overload signatures are treated as
duplicate method definitions.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterClassMethod.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterClassMethod.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed because the current observable blocker is
owned by
`issues/done/5198-support-class-method-overload-signatures-for-element-access-calls.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing class method overload issue
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

- [x] Duplicate candidates below are confirmed and this issue is superseded
- [x] Superseding issue 5198 owns class method overload signatures being treated as duplicate methods
- [x] This issue preserves failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterClassMethod.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterClassMethod.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only, no Rust code changed
- `cargo nextest run`; issue metadata only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by `issues/done/5198-support-class-method-overload-signatures-for-element-access-calls.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionRestParameterClassMethod.ts`

## Duplicate detection

- `issues/done/5198-support-class-method-overload-signatures-for-element-access-calls.md` - exact owner for valid class method overload signatures currently reported as duplicate methods
- `issues/open/5327-report-class-method-overload-wrong-implementation-name.md` - related invalid class method overload implementation-name/order diagnostics, not this valid same-name overload group
- `issues/open/5337-parse-rest-parameter-constructor-overload-signatures.md` - related rest-parameter constructor overload issue, not class methods

## Smart triage

Fresh triage shows this generated parser-syntax bucket is currently blocked
by valid class method overload-signature handling already tracked by issue
5198.

### Smart triage: Triage duplicate function: collisionRestParameterClassMethod

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/collisionRestParameterClassMethod.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterClassMethod.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterClassMethod.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
semantic_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=DuplicateFunction:1
unsupported_features=duplicate-function:1
semantic_enabled=0
```

Source context:

```ts
class c1 {
    public f4(_i: number, ...rest);
    public f4(_i: string, ...rest);
    public f4(_i: any, ...rest) {
        var _i: any;
    }
}
```

Compiler evidence:

```text
tokens: ok; includes public class methods, DotDotDot rest parameters, declare class methods, and overload signatures
ast: ok; ClassDecl c1 contains multiple Function members named f4, including bodyless overload signatures and one implementation
resolved: fails with DuplicateFunction duplicate method definition `c1.f4`
visible symbols: class c1, declare class c2, class c3, and nested _i bindings
```

TypeScript oracle evidence:

```text
ok: true
diagnostics: []
```

Superseded by:

- `issues/done/5198-support-class-method-overload-signatures-for-element-access-calls.md`

## Completion evidence

Commits:

- Superseded by `issues/done/5198-support-class-method-overload-signatures-for-element-access-calls.md`.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterClassMethod.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction class method overload blocker superseded by issue 5198
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterClassMethod.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction/duplicate-function
date: 2026-05-07
```

Remaining risks:

- After issue 5198 lands, this reference may expose later class method
  collision semantics that need a separate child issue.
