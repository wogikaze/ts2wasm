---
id: 347
title: "Parser and resolver support for direct eval and eval-code scope"
type: feature
area: frontend/semantics
class: done
priority: P3
depends_on: [336,357]
blocks: []
created: 2026-04-30
updated: 2026-05-01
completed: 2026-05-01
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

- [x] Parser detection of direct `eval(...)` call expression
- [x] Resolver scope-chain linking for eval code to caller locals
- [x] Annex B block-level function declaration binding inside eval code
- [x] Issue-linked diagnostics for unsupported indirect eval patterns

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
- [x] `cargo fmt --all --check` and focused eval validation pass; broad `cargo nextest run` is blocked only by unrelated issue 336/357 failures recorded below

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

- [x] updated: `docs/language-reference/javascript-features.md` if eval scope rules are documented

Current state:

- [x] updated: `current-state.md` if parser/resolver eval capability changes

Follow-up issues:

- [x] created: issue 348 for lowering
- [x] created: issue 349 for runtime/shim

## Notes

Parent issue: 225

The direct-eval detection must be syntactic: the callee must be the identifier `eval`, not a computed property access or local alias. Indirect eval (`window.eval(...)`) is out of scope and should be rejected with a clear diagnostic.

## Completion evidence

Completed 2026-05-01.

Commits:

- prior implementation commits recorded in progress sections
- current close-state commit

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-frontend eval -- --nocapture: pass (3)
cargo test -p ts2wasm-cli eval -- --nocapture: pass (3)
mise run update-issue-index -- --check && mise run check issues: pass
```

Remaining risks:

- Dynamic runtime eval execution remains issue 349.
- Lowering follow-up work remains issue 348.
- Broad `cargo nextest run` remains blocked by unrelated issue 336
  `reference/test262/harness` availability and issue 357 ABC451 iwasm timeout,
  as recorded in the progress evidence.

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

2026-05-01 child-347-direct-eval-close:

- Re-validated the existing parser/resolver direct-eval slice from parent base `a883f49d`.
- Focused eval gates pass, including parser recognition, caller-local eval expansion, Annex B block-function eval fixtures, and issue-347 indirect-eval diagnostics.
- Kept issue 347 open and reclassified it as blocked because the required broad `cargo nextest run` close gate still fails on unrelated open blockers, not on direct-eval parser/resolver behavior.

Validation:

```text
command: cargo fmt --all --check
result: pass

command: cargo test -p ts2wasm-frontend eval
result: pass (3 passed)

command: cargo test -p ts2wasm-cli eval
result: pass (3 passed)

command: cargo nextest run
result: fail; 447 passed, 2 failed, 4 skipped, 173 not run due fail-fast
failure 1: ts2wasm-compiler test262_preprocessor::tests::test_process_includes_and_features_inject_stubs
  BackendIo: test262 harness directory not found at /tmp/ts2wasm-347-direct-eval-close-20260430T164835Z/crates/compiler/../../reference/test262/harness
failure 2: ts2wasm-cli::m2_node_diff m2_node_diff_fixture_tests::abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
  iwasm timed out for fixtures/core-semantics/abc451-depth8-live-set.ts after 30.501s
```

Remaining blockers:

- issue 336 / its dependency chain or environment must make `reference/test262/harness` available for the broad preprocessor test.
- issue 357 must resolve the unrelated ABC451 depth-8 iwasm timeout.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/347-parser-resolver-direct-eval-scope.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
