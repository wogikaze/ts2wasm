---
id: 1301
title: "Implement Collisionrestparameterclassconstructor"
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

## Summary

Triage collisionRestParameterClassConstructor across 1 reference case and close
it after splitting the current rest-parameter constructor-overload blocker into
an implementation-ready child issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionRestParameterClassConstructor` with diagnostics: parser-syntax.
Fresh triage shows tokens and AST now succeed; the current first blocker is
`DuplicateFunction` for bodyless constructor overload signatures with rest
parameters.

Problem: class constructor overload signatures are treated as duplicate
constructor definitions.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed because the current observable blocker is
owned by `issues/open/5337-parse-rest-parameter-constructor-overload-signatures.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the rest-parameter constructor overload signature blocker into a child issue
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
- [x] Child issue 5337 contains exact reference-triage commands
- [x] Child issue 5337 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue 5337 acceptance names the exact reference path and diagnostic change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts
```

Not run:

- `cargo fmt --all --check`; issue split/close only, no Rust code changed
- `cargo nextest run`; issue split/close only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5337-parse-rest-parameter-constructor-overload-signatures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts`

## Duplicate detection

- `issues/open/5334-parse-class-constructor-overload-signatures.md` - related non-rest constructor overload signature issue, kept separate to avoid enlarging that implementation slice
- `issues/open/5337-parse-rest-parameter-constructor-overload-signatures.md` - exact owner for bodyless class constructor overload signatures with rest parameters currently reported as duplicate constructor definitions
- `issues/done/5199-report-function-overload-list-class-merge-diagnostics.md` - related top-level function/class overload merge issue, not constructor overload ownership
- `issues/done/5200-validate-top-level-function-overload-implementations.md` - related top-level function overload implementation grouping, not class constructor overload ownership
- `issues/open/5327-report-class-method-overload-wrong-implementation-name.md` - related class method overload diagnostics, explicitly out of constructor overload scope

## Smart triage

Fresh triage shows this generated parser-syntax bucket is currently blocked by
rest-parameter constructor overload handling split to issue 5337.

### Smart triage: Triage duplicate function: collisionRestParameterClassConstructor

- Issue class: `triage-needed`
- Feature label: `duplicate-function`
- Diagnostic: `DuplicateFunction` / `compiler-diagnostic`
- Path: `reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts --detail --no-dashboard-data
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
class c5 {
    constructor(_i: number, ...rest);
    constructor(_i: string, ...rest);
    constructor(_i: any, ...rest) {
        var _i: any;
    }
}
```

Compiler evidence:

```text
tokens: ok; includes class declarations, constructor members, DotDotDot rest parameters, declare class constructors, and overload signatures
ast: ok; ClassDecl bodies contain multiple Function members named constructor, including bodyless overload signatures and one implementation
resolved: fails with DuplicateFunction duplicate constructor definition
visible symbols: classes c1, c1NoError, c2, c2NoError, c3, c3NoError, c4, c4NoError, c5, c5NoError, c6, c6NoError and nested _i bindings
```

TypeScript oracle evidence:

```text
ok: true
diagnostics: []
```

Split result:

- `issues/open/5337-parse-rest-parameter-constructor-overload-signatures.md`

## Completion evidence

Commits:

- Split to `issues/open/5337-parse-rest-parameter-constructor-overload-signatures.md`.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction rest-parameter constructor overload blocker split to issue 5337
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterClassConstructor.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; DuplicateFunction/duplicate-function
date: 2026-05-07
```

Remaining risks:

- After issue 5337 lands, this reference may expose later constructor collision
  semantics that need a separate child issue.
