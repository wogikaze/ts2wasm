---
id: 1466
title: "Implement Constraints"
type: spike
area: frontend/resolver
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

Closed as superseded. Fresh triage shows
`reference/typescript/tests/cases/compiler/constraints0.ts` currently stops at
the ambient `declare var v1` name-resolution boundary already owned by
`issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`.

## Problem

Reference test results previously grouped this file under constraints. Current
compiler behavior does not reach generic constraint diagnostics because the
parser erases `declare var v1: C<A>;` and `declare var v2: C<B>;` without
leaving resolver-visible ambient value metadata. Name resolution then reports
`UnresolvedName` for `v1` in `var y = v1.x.a;`.

Problem: `constraints0.ts` is blocked by ambient `declare var` name-resolution
before generic constraint behavior can be triaged.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraints0.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraints0.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`. Do
not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage for the affected file.
- [x] Confirm the current first blocker is covered by issue 5161.
- [x] Preserve exact reproduction commands and representative diagnostic/AST
      evidence in this closed issue and the owner issue.

Out of scope:

- Direct implementation from this generated bucket.
- Generic constraint checking.
- Structural type checking for `B` against `A`.
- Property access type inference for `v1.x.a`.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_ts.rs`
- `crates/frontend/src/ast.rs`
- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`
- focused parser/resolver tests

Do not touch:

- typechecker/generic constraint logic until ambient resolution advances

## Acceptance criteria

- [x] Existing issue 5161 is confirmed as the current first-blocker owner.
- [x] This closed issue includes failing path, diagnostic code, source context,
      token evidence, compiler AST/resolved evidence, and TypeScript AST
      evidence.
- [x] Completion evidence names the exact reference path and current
      diagnostic/stdout change.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraints0.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraints0.ts
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

- [x] none; current first blocker is already tracked by issue 5161

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constraints0.ts`

## Duplicate detection

Current first blocker is covered by
`issues/done/5161-model-ambient-value-declarations-for-name-resolution.md`.

Resolution:

```text
Superseded by issue 5161. The active diagnostic is the ambient `declare var`
name-resolution gap at the later `v1.x.a` reference.
```

## Smart triage

### Smart triage: Triage name resolution: constraints0

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/constraints0.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraints0.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraints0.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Current diagnostic:

```text
UnresolvedName: unresolved name: `v1` at 204..206
```

Source context:

```ts
declare var v1: C<A>; // should work
declare var v2: C<B>; // should not work

var y = v1.x.a; // 'a' should be of type 'number'
```

Compiler evidence:

- Tokenization succeeds through interfaces `A`, `B`, `C<T extends A>`, ambient
  declarations `v1` and `v2`, and `var y = v1.x.a`.
- AST construction succeeds but only keeps `var y = v1.x.a`; the ambient vars
  are erased.
- Resolved construction fails in `resolve_names` with `UnresolvedName` for
  `v1`.
- Visible symbol extraction reports `v1`, `v2`, and `y` before the resolver
  failure.

TypeScript oracle evidence:

- TypeScript parses `declare var v1: C<A>;`, `declare var v2: C<B>;`, and
  `var y = v1.x.a;`.
- The oracle reports later TS2344 because `B` does not satisfy constraint `A`;
  that generic constraint diagnostic is not reached before this resolver
  boundary.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constraints0.ts
result: pass; current first blocker is the same ambient value name-resolution support tracked by issue 5161
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constraints0.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnresolvedName:1 unsupported_features=name-resolution:1
date: 2026-05-07
```

Remaining risks:

- After issue 5161 advances ambient value name resolution, this file may expose
  generic constraint checking, structural type checking, or property access type
  inference as later blockers.
