---
id: 347
title: "Parser and resolver support for direct eval and eval-code scope"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P3
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Add parser and resolver support to detect direct `eval` calls and model the special scope rules for eval code, including Annex B block-level function declarations inside eval.

## Problem

Direct `eval` requires the compiler to recognize `eval(...)` as a special form (not an ordinary call), and eval code must have access to the caller's local scope. Annex B block-level function declarations inside eval have special hoisting and binding rules that differ from ordinary block-scoped functions.

Problem: Parser and resolver do not detect direct eval calls or model eval-code scope.

## Current failure

```sh
tmp=/tmp/ts2wasm-347-eval-scope.ts
printf 'function f() { let x = 1; eval("x"); }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-347-eval-scope.wasm
```

Current result: `eval` is treated as an unresolved identifier or ordinary function call, and eval-code scope analysis is not performed.

## Desired final state

The parser and resolver can:
1. Detect direct `eval(...)` calls at the syntactic level
2. Flag eval code for special scope treatment
3. Resolve variable references in eval code against the caller's local scope
4. Model Annex B block-level function declaration hoisting inside eval code

## Scope

In scope:

- [ ] Parser detection of direct `eval(...)` call expression
- [ ] Resolver scope-chain linking for eval code to caller locals
- [ ] Annex B block-level function declaration binding inside eval code
- [ ] Issue-linked diagnostics for unsupported indirect eval patterns

Out of scope:

- Runtime execution of eval code (issue 349)
- Lowering of eval code AST (issue 348)
- Full host global environment behavior beyond caller-local scope

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/frontend/src/parser/`
- `crates/cli/tests/`
- `fixtures/core-semantics/eval*`

Do not touch:

- `crates/backend-wasm/src/`
- `crates/ir/src/lowered/`

## Acceptance criteria

- [x] Parser fixture proves `eval("...")` is recognized as a direct eval call
- [x] Resolver fixture proves eval code can resolve caller-local variable references
- [x] Diagnostic fixture proves unsupported indirect eval is rejected with issue-linked error
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli eval
cargo test -p ts2wasm-frontend eval
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/language-reference/javascript-features.md` if eval scope rules are documented

Current state:

- [ ] updated: `current-state.md` if parser/resolver eval capability changes

Follow-up issues:

- [ ] created: issue 348 for lowering
- [ ] created: issue 349 for runtime/shim

## Notes

Parent issue: 225

The direct-eval detection must be syntactic: the callee must be the identifier `eval`, not a computed property access or local alias. Indirect eval (`window.eval(...)`) is out of scope and should be rejected with a clear diagnostic.

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

## Progress evidence

2026-04-30 child-347:

- Implemented syntactic direct eval recognition on `Expr::Call` and parser expansion for static-string direct `eval(...)` expression statements.
- Added caller-scope eval fixture `fixtures/core-semantics/direct-eval-caller-local.ts`, which resolves an eval-code assignment against the caller function local and matches Node output under the CLI eval filter.
- Added indirect eval diagnostic fixture `fixtures/core-semantics/direct-eval-indirect-unsupported.ts`, which reports `issue-347` for `globalThis.eval(...)`.
- Kept backend and IR lowering out of scope; dynamic runtime eval execution remains tracked by issue 349, and eval lowering follow-up remains tracked by issue 348.

Validation:

```text
command: cargo fmt --all --check
result: pass

command: cargo test -p ts2wasm-frontend eval
result: pass (3 passed)

command: cargo test -p ts2wasm-cli eval
result: pass (3 passed)

command: cargo nextest run
result: blocked after 436/617 completed; unrelated pre-existing environment failure:
  test262_preprocessor::tests::test_process_includes_and_features_inject_stubs
  BackendIo: test262 harness directory not found at reference/test262/harness

command: mise run update-issue-index -- --check
result: pass

command: mise run check issues
result: pass

command: mise run check agent-state
result: pass
```

Remaining risks:

- Full DONE close is deferred until `cargo nextest run` can pass in an environment with `reference/test262/harness` available or the pre-existing harness requirement is otherwise resolved.
