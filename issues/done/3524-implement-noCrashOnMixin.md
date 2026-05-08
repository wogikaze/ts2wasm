---
id: 3524
title: "Implement Nocrashonmixin"
type: spike
area: ir/resolver
class: superseded
priority: P1
depends_on: [5252]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5252-support-call-expression-class-heritage.md`. Fresh triage for both representatives shows the current blocker is the existing call-expression class heritage gate for `class CrashTrigger extends Mixin(Empty)`.

## Problem

Reference test results originally showed 2 cases failing in directory `noCrashOnMixin` with diagnostics: class. Fresh focused evidence shows both files now parse into AST and fail during resolver/builtin validation:

```text
UnsupportedSyntax: only simple inheritance (extends ClassName) is supported
```

Problem: this generated bucket is not a standalone implementation order. The current observable blocker is already tracked by issue 5252.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnMixin.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnMixin2.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnMixin.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnMixin2.ts --detail --no-dashboard-data
```

Focused coverage for each file:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Compiler evidence:

```text
tokens: ok through Abstract, Concrete, Constructor<T>, Mixin, Empty, and CrashTrigger
ast: ok; ClassDecl CrashTrigger extends Call(callee=Ident Mixin, args=[Ident Empty])
resolved: fails in validate_ast / builtin resolution with only simple inheritance diagnostic
```

TypeScript oracle:

```text
noCrashOnMixin.ts: TS2674 protected constructor access for new Concrete()
noCrashOnMixin2.ts: TS2370 rest parameter must be array type, TS1047 optional rest parameter, TS2545 invalid mixin constructor, TS2674 protected constructor access
```

## Desired final state

This generated bucket is closed as superseded by issue 5252. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage for both representatives
- [x] Confirm issue 5252 covers the current `extends Mixin(Empty)` call-expression heritage blocker
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue
- [x] Update issue 5252 with the noCrashOnMixin evidence

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

- [x] Duplicate candidates below are confirmed as superseded by issue 5252
- [x] Superseding issue contains exact `reference-triage` commands
- [x] Closed issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnMixin.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCrashOnMixin2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnMixin.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCrashOnMixin2.ts
```

Not run:

- `cargo fmt --all --check` / `cargo nextest run`: issue metadata-only supersession; no Rust implementation changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing: `issues/open/5252-support-call-expression-class-heritage.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noCrashOnMixin.ts`
- `reference/typescript/tests/cases/compiler/noCrashOnMixin2.ts`

## Duplicate detection

- `issues/open/5252-support-call-expression-class-heritage.md`: exact owner for `only simple inheritance (extends ClassName) is supported` on class heritage call expressions.

## Smart triage

Generated on 2026-05-08.

```text
noCrashOnMixin.ts: UnsupportedSyntax only simple inheritance (extends ClassName) is supported
noCrashOnMixin2.ts: UnsupportedSyntax only simple inheritance (extends ClassName) is supported
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
