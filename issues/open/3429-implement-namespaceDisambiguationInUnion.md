---
id: 3429
title: "Implement Namespacedisambiguationinunion"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed this generated `import-export` bucket because the current compiler build
now passes. The remaining TypeScript oracle mismatch is split to
`issues/open/5441-report-namespaced-union-literal-assignment-diagnostic.md`.

## Problem

Reference test results show 1 cases fail in directory `namespaceDisambiguationInUnion` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Fresh coverage shows `namespaceDisambiguationInUnion.ts` no longer fails with
`import-export`; it is a build pass. TypeScript still reports TS2322 for
assigning an inferred `{ type: string }` object to `Foo.Yep | Bar.Yep`.

Problem: the stale generated import/export blocker is gone, and the remaining
semantic parity gap belongs to a focused namespaced type-alias assignment
diagnostic issue.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts --detail
```

## Desired final state

This generated bucket is closed. Implement semantic parity from
`issues/open/5441-report-namespaced-union-literal-assignment-diagnostic.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the stale import/export blocker is gone
- [x] Split the remaining semantic oracle mismatch into a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `issues/open/5441-report-namespaced-union-literal-assignment-diagnostic.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5441-report-namespaced-union-literal-assignment-diagnostic.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts`

## Duplicate detection

- No exact owner existed for the first TS2322 diagnostic on
  `const val1: Foo.Yep | Bar.Yep = x;`, so issue 5441 was created.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts --detail --no-dashboard-data

result:
executed=1
build_pass=1
unsupported=0
blocked=0
semantic_enabled=0
reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts: build_pass
```

Fresh focused triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts

result:
BuildPass: ts2wasm build succeeded
TypeScript oracle reports TS2322 at `val1` and a later TS2322 at `val2`.
```

Source context:

```ts
namespace Foo {
  export type Yep = { type: "foo.yep" };
}

namespace Bar {
  export type Yep = { type: "bar.yep" };
}

const x = { type: "wat.nup" };
const val1: Foo.Yep | Bar.Yep = x;

const y = [{ type: "a" }, { type: "b" }];
const val2: [Foo.Yep, Bar.Yep] = y;
```

Compiler evidence:

```text
tokens: ok through both namespaces, exported type aliases, x/val1, y/val2
ast: ok; namespace type aliases and type annotations are erased from retained runtime AST
resolved: ok; retained statements are x object literal, val1 = x, y array literal, val2 = y
coverage: executed=1, build_pass=1, unsupported=0
```

TypeScript oracle evidence:

```text
TS2322 at val1: Type '{ type: string; }' is not assignable to type 'Foo.Yep | Bar.Yep'.
TS2322 at val2: Type '{ type: string; }[]' is not assignable to type '[Foo.Yep, Bar.Yep]'.
```

## Completion evidence

Closed as stale import/export bucket; the current semantic mismatch was split to
issue 5441.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespaceDisambiguationInUnion.ts
result: pass; compiler build-passes, TypeScript oracle reports TS2322 split to issue 5441
date: 2026-05-08
```

Remaining risks:

- The later fixed-tuple assignment diagnostic on `val2` may need a narrower
  follow-up after issue 5441 stops hiding the first oracle mismatch.
