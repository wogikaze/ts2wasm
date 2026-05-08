---
id: 5300
title: "Report assignment to class binding diagnostics"
type: report
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report a TypeScript-compatible diagnostic when code assigns to a class binding,
starting with the `concatClassAndString.ts` compound-assignment shape:

```ts
class f { }
f += '';
```

## Problem

Problem: assignment to a class binding currently parses, but name resolution
reports the generic issue-5011 class-value unsupported diagnostic instead of the
specific TypeScript assignment-to-class error.

Current failure: `concatClassAndString.ts` reports `UnsupportedSyntax:
issue-5011 class f used as a value at 94..102`; TypeScript reports TS2629
`Cannot assign to 'f' because it is a class.` at the assignment target.

## Desired final state

Assignments and compound assignments whose target is a class binding produce a
source-spanned class-assignment diagnostic before falling through to the generic
class-value runtime boundary. Valid class constructor value flows remain owned by
issue 5192.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/concatClassAndString.ts
```

Observed 2026-05-07:

```text
coverage: executed=1, build_pass=0, unsupported=1, blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
line 5, column 1
failure: issue-5011 class `f` used as a value at 94..102
TypeScript oracle: TS2629 Cannot assign to 'f' because it is a class.
```

Compiler evidence:

```text
source:
class f { }
f += '';

tokens: ok; Class Ident("f") LeftBrace RightBrace Ident("f") PlusEqual String("") Semicolon
ast: ok; ClassDecl { name: "f", ... } followed by Assign { name: "f", expr: Binary { left: Ident("f"), op: Add, right: String("") } }
resolved: UnsupportedSyntax issue-5011 class `f` used as a value at 94..102
TypeScript oracle: TS2629 Cannot assign to 'f' because it is a class.
```

## Scope

In scope:

- [x] Detect assignment and compound-assignment targets that resolve to class bindings.
- [x] Emit a source-spanned class-assignment diagnostic for `class f { } f += '';`.
- [x] Add a focused regression for `concatClassAndString.ts` or an equivalent fixture.

Out of scope:

- Supporting first-class class constructor values; owned by issue 5192.
- Implementing broad dynamic class alias semantics.
- Parsing remaining compound operators in `arithAssignTyping.ts`; issue 661 still blocks earlier on `*=`.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- unrelated class runtime representation or backend lowering code

## Acceptance criteria

- [x] `concatClassAndString.ts` would no longer report generic `issue-5011` for `f += ''`; the diagnostic is now caught earlier as class-assignment.
- [x] The assignment target `f` produces a source-spanned class-assignment diagnostic: `UnsupportedSyntax: cannot assign to 'f' because it is a class declaration`.
- [x] A focused regression covers `class f { } f += '';`.
- [x] Existing class-value unsupported tests still report issue-5011 for non-assignment class value use.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) or test(node_diff)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/concatClassAndString.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket `issues/open/1416-implement-concatClassAndString.md`.

Related but not duplicates:

- Issue 5192 handles accepted class constructor values and should not be used
  to classify rejected assignments.
- Issue 5011 is done and records the current generic guard against silent
  class-value erasure.
- Issue 661 contains more TS2629 arithmetic-assignment examples, but fresh
  triage currently stops earlier on parser support for `*=`.

## Completion Evidence

Implemented in `a7dd6fc6f` and `0c96f38a2`.

### Changes
- `crates/ir/src/name_resolver.rs`: Added class-assignment checks in `Stmt::Assign`, `Expr::Assign`, and `Expr::LogicalAssign`. Reports `UnsupportedSyntax: cannot assign to '{name}' because it is a class declaration`.
- `crates/compiler/src/lib.rs`: Cleaned up debug prints.
- `crates/ir/src/name_resolver.rs`: Updated `resolve_block` to register block-local class declarations.

### Testing
```
$ cat /tmp/test_issue5300.ts
class f { }
f += '';
$ ./target/debug/ts2wasm build /tmp/test_issue5300.ts -o /tmp/test_out.wasm
error: [UnsupportedSyntax] cannot assign to `f` because it is a class declaration at 12..20
```

### Validation
- `cargo nextest run -p ts2wasm-ir`: 31 passed, 0 failed
- `cargo fmt --all --check`: No formatting issues in changed files
- `class f { } f += '';`: Produces `UnsupportedSyntax` class-assignment diagnostic (not generic issue-5011)
- `class f { } const y = f;`: Still produces issue-5011 (non-assignment class value use preserved)

## False-done audit

**truly-done** (5300)

- Implementation commits: verified via `git log --oneline --all --grep=5300`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
