---
id: 1253
title: "Implement Clodulegenericonselfmember"
type: spike
area: frontend/semantics
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

Closed as stale. Fresh triage and focused coverage show
`reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts` now
build-passes, so there is no current type-system blocker to split into a child
issue.

## Problem

Reference test results previously showed 1 case failing in directory
`cloduleGenericOnSelfMember` with diagnostics: type-system. Fresh triage shows
the compiler now build-passes the reference path.

Problem: the generated type-system bucket is stale. TypeScript still reports
TS2564 for an uninitialized class property, but semantic diagnostics are not
enabled in this coverage window and no current compiler build blocker remains.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as stale because the representative path now
reports `build_pass`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the representative case is now build-pass with no active compiler blocker
- [x] Close as stale build-pass instead of creating a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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

- [x] Duplicate candidates below are confirmed; no separate issue is needed
- [x] No child issue needed because the representative case now build-passes
- [x] This issue includes path, diagnostic status, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and build-pass result

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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts
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

- [x] none; current compiler build has no type-system blocker on this path

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts`

## Duplicate detection

No exact implementation child is created because the original type-system
compiler blocker no longer reproduces.

Resolution:

```text
The original type-system blocker is stale. The reference window now reports build_pass with semantic checking disabled.
```

## Smart triage

### Smart triage: Build pass: cloduleGenericOnSelfMember

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
unsupported_features=
```

Source context:

```ts
class ServiceBase<T> {
    field: T;
}
class Service extends ServiceBase<typeof Service.Base> {
}
namespace Service {
    export const Base = {
        name: "1",
        value: 5
    };
}
```

Compiler evidence:

```text
tokens: ok; generic class declaration, typeof qualified type argument, namespace Service, and export const Base are tokenized
ast: ok; retained AST contains ServiceBase and Service class declarations
resolved: ok; Service extends ServiceBase resolves
```

TypeScript oracle evidence:

```text
TS2564: Property 'field' has no initializer and is not definitely assigned in the constructor.
binding hint: Base has type { name: string; value: number; }
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; build succeeded and original type-system blocker is stale
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleGenericOnSelfMember.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; executed=1, build_pass=1, unsupported=0
date: 2026-05-07
```

Remaining risks:

- TypeScript still reports TS2564 strict property initialization; that is a
  future semantic parity gap, not the generated type-system blocker closed here.
