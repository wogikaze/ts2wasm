---
id: 3545
title: "Implement Noimplicitanymissinggetaccessor"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed after refreshed evidence showed this generated class-accessor bucket is
stale. The representative now build-passes in ts2wasm.

## Problem

Reference test results previously showed 1 case failing in
`noImplicitAnyMissingGetAccessor` with diagnostics: class-accessor.

Fresh triage on 2026-05-08 shows the current compiler tokenizes, parses,
resolves, and builds the abstract setter and concrete setter without an
unsupported diagnostic.

Problem: `noImplicitAnyMissingGetAccessor.ts` no longer has a current build
failure in the representative coverage window.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyMissingGetAccessor.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyMissingGetAccessor.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=1 unsupported=0
triage: BuildPass ts2wasm build succeeded
```

## Desired final state

This generated bucket is closed as stale. Do not implement directly from this
bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence
- [x] Close as stale build-pass evidence

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
- [x] Exact focused `reference-triage` command is recorded below
- [x] Evidence includes path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and build-pass result

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyMissingGetAccessor.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyMissingGetAccessor.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only.
- `cargo nextest run`; issue metadata only.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyMissingGetAccessor.ts`

## Duplicate detection

- Fresh triage found no current compiler blocker.
- No child issue was created because both ts2wasm build and the TypeScript
  oracle report success for the representative.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Build pass: noImplicitAnyMissingGetAccessor

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/noImplicitAnyMissingGetAccessor.ts
```

Focused coverage:

```text
suite=tsc
executed=1
build_pass=1
unsupported=0
semantic_enabled=0

reference/typescript/tests/cases/compiler/noImplicitAnyMissingGetAccessor.ts: build_pass
```

Source context:

```ts
abstract class Parent
{
    public abstract set message(str);
}

class Child extends Parent {
    _x: any;
    public set message(str) {
        this._x = str;
    }
}
```

Compiler evidence:

```text
tokens: ok; includes public abstract set message(str); and public set message(str) body
ast: ok; Parent ClassDecl has Function "set message" with empty body; Child has Function "set message" with PropertyAssign this._x = str
resolved: ok; Parent and Child ClassDecls resolve, Child extends Parent, setter params are present
visible symbols: class Parent, class Child
```

TypeScript oracle:

```text
diagnostics: []
parameter str in abstract setter: any
parameter str in concrete setter: any
```

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyMissingGetAccessor.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyMissingGetAccessor.ts
result: pass; BuildPass with no compiler blocker
date: 2026-05-08
```

Remaining risks:

- none
