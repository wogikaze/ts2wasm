---
id: 5178
title: "Parse bitwise compound assignment operators"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

`bitwiseCompoundAssignmentOperators.ts` stops at the first `a ^= a;` because the frontend does not represent bitwise compound assignment operators as assignment expressions.

## Problem

The lexer currently emits `Caret` followed by `Equal` for `^=`, and the parser treats the expression as if it started with ordinary bitwise XOR before failing on the `Equal` token. The reference case cannot advance to the TypeScript oracle diagnostics for invalid boolean/number operand combinations.

Problem: bitwise compound assignment operators `^=`, `&=`, and `|=` fail in parser/frontend syntax before semantic diagnostics can be compared.

## Current failure

Reference triage:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts
```

Current compiler diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Equal, span: Span { start: 47, end: 48 } }) at 49..50
```

Representative source:

```ts
var a = true;
var b = 1;
a ^= a;
a = true;
b ^= b;
b = 1;
a ^= b;
```

Compiler evidence:

- Token dump for the first failing operator emits `Ident("a")`, `Caret`, `Equal`, `Ident("a")`.
- AST/resolved construction fails at the `Equal` token before representing `a ^= a;`.
- Visible symbols before failure include `a` initialized to `true` and `b` initialized to `1`.

TypeScript oracle evidence:

```text
TS2447: The '^=' operator is not allowed for boolean types. Consider using '!==' instead.
TS2362: The left-hand side of an arithmetic operation must be of type 'any', 'number', 'bigint' or an enum type.
TS2363: The right-hand side of an arithmetic operation must be of type 'any', 'number', 'bigint' or an enum type.
```

The same file also exercises `&=` and `|=` with boolean/number combinations.

## Desired final state

The frontend accepts `^=`, `&=`, and `|=` as assignment-expression syntax for identifier targets and either lowers supported numeric cases or reports later source-spanned semantic diagnostics. The representative case should no longer fail because `^=` was split into `Caret` plus `Equal`.

## Scope

In scope:

- [x] Tokenize or parser-detect `^=`, `&=`, and `|=` as bitwise compound assignment operators.
- [x] Represent identifier-target bitwise compound assignment in the AST/dump path.
- [x] Preserve existing ordinary binary `^`, `&`, and `|` parsing.
- [x] Add focused parser/frontend coverage for `a ^= a;`, `c &= c;`, and `e |= e;`.
- [x] Re-run representative triage and confirm the current `Equal` parser blocker is gone.

Out of scope:

- TypeScript operand type diagnostics for every boolean/number combination in the reference file.
- Runtime lowering for member or computed assignment targets.
- BigInt bitwise compound assignment.
- Logical assignment operators `&&=`, `||=`, and `??=`; those are already separate implemented paths.
- Exponentiation compound assignment `**=` owned by issue `5164`.

## Affected paths

Expected:

- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/lexer_tokens.rs`
- `crates/frontend/src/ast.rs`
- `crates/frontend/src/parser/expressions_main.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/compiler/src/dump.rs`
- `crates/ir/src/name_resolver.rs`

Do not touch:

- `crates/backend-wasm/src/` unless the representative triage advances past parsing and proves a backend-specific blocker.
- Logical assignment lowering paths.

## Acceptance criteria

- [x] `a ^= a;` parses without `unsupported expression: ... Equal`.
- [x] `c &= c;` and `e |= e;` parse through the same bitwise compound assignment representation.
- [x] Ordinary binary `a ^ b`, `c & d`, and `e | f` parsing remains unchanged.
- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts` no longer reports the current `Equal` parser diagnostic.
- [x] A focused parser/frontend regression covers all three bitwise compound assignment operators.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend compound
cargo nextest run -p ts2wasm-ir
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts
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

- [x] created: `issues/done/5221-support-bitwise-and-xor-binary-lowering.md`

## Notes

Split from generated bucket `1065` on 2026-05-06. Issue `661` also contains arithmetic-assignment typing evidence, but its first current parser blocker is `*=`; this issue is the bitwise compound assignment syntax slice for `^=`, `&=`, and `|=`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-frontend compound
result: pass (5 passed)
date: 2026-05-06

command: cargo nextest run -p ts2wasm-ir
result: pass (26 passed)
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/bitwiseCompoundAssignmentOperators.ts
result: parser/AST pass; advanced to `binary operator BitwiseXor not yet supported`
date: 2026-05-06
```

Remaining risks:

- Later AND/XOR lowering is split to issue 5221.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

