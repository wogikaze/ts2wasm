# DECISIONS.md

## Recursive Run Index

- `run-173-object-seal`: Register Object.seal in builtin dispatch (commit 7a08bfa78, merged)
  - Phase 4 validated: 1 new test passing, 0 regressions
  - Decision: Follow exact Object.preventExtensions pattern.
    Emitter reuses SEALED flag bit. No RuntimeFn deps needed.
- `run-196-promise-substrate`: Register promise_basic_matches_node_output acceptance test
  - No WAT/IR changes needed — PromiseConstructor/PromiseResolve/PromiseReject runtime already implemented
  - Decision: add fixture + test only; no production code changes
- `run-172-async-exception`: Enable semantic_diff_async_exception test (commit 3cbf58915)
  - Async exception handling already works; test was preemptively #[ignore]d.
- `run-156-module-augmentation`: Fix build_smoke_module_augmentation test (commit c28347041)
  - Changed from assert_is_ok to assert_is_err with diagnostic message check.
- `run-195-decorator-parser`: Erase TypeScript decorator syntax at statement level (commit 3b94252b8)
  - Changed `decorator_before_class_declaration` to call `self.statement()` after consuming `@expr` prefix
  - Only statement-level decorators (`@X class Y {}`); class-expression decorators still rejected
  - No runtime semantics; pure parser erasure
- `arch-design-review-2026-05-11`: Comprehensive architectural analysis of ts2wasm
  - Analysis covers 25 sections: coupling points, target architecture, LLM-friendly sizing, refactoring methodology
  - 8 priority-ordered refactoring items identified (P1-P8)
  - Decision: Use recursive-mode for each P-item individually; start with P1 (Span/Diagnostic extraction)
  - Decision: Each P-item gets its own worktree and commit
  - Full analysis captured in `.recursive/run/arch-design-review-2026-05-11/reference/`
