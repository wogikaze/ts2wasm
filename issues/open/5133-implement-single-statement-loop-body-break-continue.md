---
id: 5133
title: "Implement single-statement loop bodies for break and continue"
type: feature
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: [707, 708, 418]
status: done
created: 2026-05-06
updated: 2026-05-06
completed: 2026-05-06
---

## Summary

Allow loop statements to parse a single non-block statement body when that body
is `break` or `continue`, covering the narrow ASI reference cases split from
issues 707 and 708.

## Problem

The parser currently requires `{ ... }` after `while (...)`, so valid
JavaScript/TypeScript statements such as `while (true) break` and
`while (true) continue` fail before lowering.

Problem: `asiBreak.ts` and `asiContinue.ts` fail with `expected LeftBrace` at
the `break` / `continue` token even though the TypeScript oracle accepts both.

## Current failure

Current diagnostics:

```text
asiBreak.ts: error: [UnsupportedSyntax] expected LeftBrace, got Some(Break) at 35..40
asiContinue.ts: error: [UnsupportedSyntax] expected LeftBrace, got Some(Continue) at 35..43
```

TypeScript AST evidence in issues 707 and 708 shows `WhileStatement` with a
`BreakStatement` or `ContinueStatement` body.

## Desired final state

The parser accepts the narrow single-statement loop-body forms needed by these
reference cases, preserving existing block-bodied loop behavior and existing
invalid break/continue diagnostics outside loop bodies.

## Scope

In scope:

- [x] Parse `while (true) break` as a while statement whose body contains one `break`.
- [x] Parse `while (true) continue` as a while statement whose body contains one `continue`.
- [x] Add focused parser coverage for both forms.

Out of scope:

- Single-statement loop bodies for arbitrary statements.
- Broader ASI policy beyond the two split reference cases.
- Labeled break/continue semantics, already covered by issue 209.
- Runtime loop semantics changes unrelated to parsing the body shape.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- `fixtures/` only if a compiler-level regression fixture is needed
- `issues/open/5133-implement-single-statement-loop-body-break-continue.md`

Do not touch:

- runtime/backend memory code
- BigInt files
- reference harness scripts unless the triage command itself is wrong

## Acceptance criteria

- [x] `asiBreak.ts` no longer reports `expected LeftBrace, got Some(Break)`.
- [x] `asiContinue.ts` no longer reports `expected LeftBrace, got Some(Continue)`.
- [x] A focused parser test covers both `while (true) break` and `while (true) continue`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo test -p ts2wasm-frontend parser
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiBreak.ts
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiContinue.ts
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run check issues
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

This issue intentionally starts with the `break` / `continue` body forms from
the two tsc ASI reference cases. If broader single-statement loop bodies are
needed, split them separately from issue 418.

## Completion evidence

Commits:

- `3a5f000e chore: commit pending issue moves and fixture changes`

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo test -p ts2wasm-frontend parser -- --nocapture
result: pass; 133 passed
date: 2026-05-06

command: cargo run -q -p ts2wasm-cli -- dump --ast reference/typescript/tests/cases/compiler/asiBreak.ts
result: pass; AST is While body [Break]; no expected LeftBrace diagnostic
date: 2026-05-06

command: cargo run -q -p ts2wasm-cli -- dump --ast reference/typescript/tests/cases/compiler/asiContinue.ts
result: pass; AST is While body [Continue]; no expected LeftBrace diagnostic
date: 2026-05-06

command: cargo run -q -p ts2wasm-cli -- check reference/typescript/tests/cases/compiler/asiBreak.ts
result: pass
date: 2026-05-06

command: cargo run -q -p ts2wasm-cli -- check reference/typescript/tests/cases/compiler/asiContinue.ts
result: pass
date: 2026-05-06

command: timeout 90 mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiBreak.ts
result: interrupted after updated report; parser/token/AST/resolved dumps pass and diagnostic changed from expected LeftBrace to BackendIo/wat2wasm
date: 2026-05-06

command: timeout 90 mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/asiContinue.ts
result: interrupted after updated report; parser/token/AST/resolved dumps pass and diagnostic changed from expected LeftBrace to BackendIo/wat2wasm
date: 2026-05-06
```

Remaining risks:

- Reference triage still reports a downstream BackendIo/wat2wasm category after parsing succeeds; that is outside this parser-only slice.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

