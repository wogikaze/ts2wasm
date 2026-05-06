---
id: 5223
title: "Report Iterator type-only value-use diagnostics"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

After ambient generic variable declarations parse, `builtinIterator.ts` reaches
`Iterator.from(...)` value uses and reports a generic `UnresolvedName` for
`Iterator`. TypeScript accepts the syntax and reports TS2693 because `Iterator`
is only available as a type in this reference case.

## Problem

The resolver currently treats `Iterator` value uses as an ordinary unresolved
runtime name, which hides the narrower TypeScript type-only value-use
diagnostic exposed by `builtinIterator.ts`.

Problem: `Iterator.from(...)` in `builtinIterator.ts` reports generic `UnresolvedName` instead of a source-spanned type-only value-use diagnostic.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
```

Current compiler diagnostic after issue 5222:

```text
Diagnostic: UnresolvedName / resolver-symbol
message: unresolved name: `Iterator` at 54..62
line 4 column 18
```

Representative source:

```ts
const iterator = Iterator.from([0, 1, 2]);
```

TypeScript oracle:

```text
TS2693: 'Iterator' only refers to a type, but is being used as a value here.
```

## Desired final state

The resolver recognizes value-position uses of known type-only ambient names
such as `Iterator` and emits a precise, source-spanned diagnostic before the
generic unresolved-name path. The diagnostic should let reference triage
distinguish TypeScript type/value errors from genuine missing runtime globals.

## Scope

In scope:

- [x] Add a type-only value-use diagnostic path for `Iterator` value-position identifiers.
- [x] Preserve ordinary `UnresolvedName` for genuinely unknown identifiers.
- [x] Add focused resolver or CLI coverage for `Iterator.from([0])`.
- [x] Re-run `builtinIterator.ts` triage and record the next narrower blocker.

Out of scope:

- Iterator helper runtime implementation.
- Full lib.d.ts type checking or declaration merging.
- Supporting `Iterator.from` as a runtime builtin.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/ir/src/name_resolver_tests.rs`
- `fixtures/` or `crates/cli/tests/`

Do not touch:

- `crates/backend-wasm/src/`
- Iterator runtime/builtin implementation

## Acceptance criteria

- [x] `builtinIterator.ts` no longer reports generic `UnresolvedName` for the first `Iterator` value use.
- [x] A focused test covers `Iterator.from([0])` and asserts the narrower diagnostic.
- [x] Existing unresolved-name coverage for unknown identifiers still passes.
- [x] Follow-up work is represented if triage advances to iterator helper runtime or member diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir name_resolver
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
```

Impacted commands:

```sh
mise run check issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected
- [x] diagnostic state recorded in this issue

Follow-up issues:

- [x] none

## Notes

Split while closing issue 5222. Issue 5203 covers an indexed `new any[1]`
type-only callee diagnostic; this issue is the separate `Iterator` value-use
path exposed by `builtinIterator.ts`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit: ir: report Iterator type-only value use

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-ir name_resolver
result: pass; 20 tests passed
date: 2026-05-06

command: cargo build -q -p ts2wasm-cli && python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/builtinIterator.ts
result: pass for issue 5223; first Iterator value use now reports TypeScriptTypeCheck TS2693 instead of generic UnresolvedName
date: 2026-05-06
```

Remaining risks:

- `builtinIterator.ts` still contains later iterator helper and interface diagnostics from the TypeScript oracle; those are not runtime support tasks in this diagnostic slice.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

