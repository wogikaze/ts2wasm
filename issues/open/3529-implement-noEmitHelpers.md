---
id: 3529
title: "Implement Noemithelpers"
type: spike
area: frontend/lexer
class: superseded
priority: P1
depends_on: [5276]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Superseded by `issues/open/5276-report-class-declaration-decorator-boundary.md`. Fresh triage shows the current blocker is the class declaration decorator `@decorator` before `class A`.

## Problem

Reference test results originally showed 1 case failing in directory `noEmitHelpers` with diagnostics: parser-syntax. Fresh triage reports:

```text
UnsupportedSyntax: unsupported character: @ at 143..144
```

Problem: this generated bucket is not a standalone implementation order. The current class declaration decorator lexer boundary is already tracked by issue 5276.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noEmitHelpers2.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noEmitHelpers2.ts --detail --no-dashboard-data
```

Focused coverage:

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
tokens: fail at `@` before class declaration
ast/resolved: same lexer failure
visible symbol before failure: decorator binding from `declare var decorator: any;`
```

TypeScript oracle:

```text
TS1206: Decorators are not valid here. at the constructor parameter decorator
TypeScript AST: ClassDeclaration with Decorator `@decorator`
```

## Desired final state

This generated bucket is closed as superseded by issue 5276. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage evidence
- [x] Confirm issue 5276 covers the current class declaration decorator boundary
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue
- [x] Record the later parameter-decorator TS1206 risk without creating a premature duplicate

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

- [x] Duplicate candidates below are confirmed as superseded by issue 5276
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noEmitHelpers2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noEmitHelpers2.ts
```

Not run:

- `cargo fmt --all --check` / `cargo nextest run`: issue metadata-only supersession; no Rust implementation changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing: `issues/open/5276-report-class-declaration-decorator-boundary.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noEmitHelpers2.ts`

## Duplicate detection

- `issues/open/5276-report-class-declaration-decorator-boundary.md`: exact owner for the current class declaration decorator lexer boundary.
- Later constructor parameter decorator diagnostics should be revisited after issue 5276 advances this reference.

## Smart triage

Generated on 2026-05-08.

```text
Feature label: parser-syntax
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: unsupported character: @ at 143..144
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
