---
id: 5142
title: "Support class method calls on new-expression receivers"
type: feature
area: ir/lowered
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Implement the narrow method-call receiver slice where a class instance is constructed inline and immediately used as the method receiver, such as `new C().g()`.

## Problem

The parser already builds a `Call(Member(New(Ident("C")), "g"))` AST for the representative TypeScript reference case, and name resolution reaches lowering. Lowering then rejects the method call because issue-211 currently requires an identifier receiver.

Problem: method calls on new-expression receivers currently fail with `UnsupportedSyntax`, even when the class and method are local and the method body is otherwise supported.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoid.ts
```

Current diagnostic:

```text
error: [UnsupportedSyntax] issue-211: method `g` requires an identifier receiver at 228..239
```

Source context:

```text
15 |     }
16 | }
17 |
18 | var z=new C().g(); // error void fn
19 | var N=new f();  // ok with void fn
20 |
```

Relevant compiler evidence:

```text
AST: Let z = Call(Member(New(Ident("C"), args=[]), property="g"), args=[])
Pipeline: validate_ast -> module_graph -> resolve_names -> resolve_builtins -> build_typed_ir -> lower_program
Failure: lower_program reports issue-211 because method `g` has a non-identifier receiver.
TypeScript oracle: ok, no diagnostics; binding `z` has type `void`.
```

## Desired final state

The compiler lowers `new C().g()` when:

- `C` is a local class declaration already supported by existing `new C()` lowering;
- `g` is a supported instance method on `C`;
- the receiver expression is evaluated once and bound to the method call receiver for the call duration;
- the method result type and runtime behavior match the existing identifier-receiver method call path.

## Scope

In scope:

- [x] Add the smallest lowering path for `ResolvedExpr::MethodCall` whose object is `ResolvedExpr::New` for a known local class.
- [x] Preserve single evaluation of the `new C()` receiver before invoking `g`.
- [x] Add a focused fixture equivalent to `class C { g() {} } var z = new C().g();`.
- [x] Re-run the `avoid.ts` triage and confirm it advances past the issue-211 identifier-receiver diagnostic.

Out of scope:

- General arbitrary expression receivers such as `factory().g()`.
- Computed method names, optional chaining, or extracted function-valued local calls.
- Full prototype/builtin method-call semantics covered by broad issue 435.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver_expr.rs`
- `crates/ir/src/lowered/resolver_extra.rs`
- `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`
- `fixtures/core-semantics/`

Do not touch:

- parser grammar unless a focused regression proves the AST shape above is no longer produced
- unrelated builtin method-call implementations

## Acceptance criteria

- [x] A focused Node/iwasm fixture for `new C().g()` builds and matches Node output.
- [x] The fixture proves the receiver is evaluated once before method invocation.
- [x] `mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoid.ts` no longer reports `method \`g\` requires an identifier receiver`.
- [x] Existing unsupported diagnostics for extracted method calls remain source-spanned.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli class_new_expression_method_call
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/avoid.ts
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/avoid.ts --detail
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

Split from generated bucket `issues/open/1013-implement-avoid.md`.

Related broad parent:

- `issues/open/435-implement-method-call.md`

## Completion evidence

Commits:

- `5a6b3953` chore: commit class method call and void operator implementations

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/avoid.ts
result: pass; triage advanced past issue-211 identifier-receiver diagnostic to BackendIo
date: 2026-05-06
```

Remaining risks:

- The reference case still has a downstream BackendIo failure outside this method-call receiver slice.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

