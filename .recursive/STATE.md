# STATE.md

## Current State (2026-05-13)

- Coverage expansion epics completed (coverage-epics-2026-05-12)
  - 6 epics implemented in parallel child worktrees, parent-merged to master
  - Builtin API Math/JSON coverage expanded (Math.cbrt, Math.clz32, Math.imul, etc.)
  - Class derived fields + fields-methods fixtures added, all 127 class tests passing
  - Promise.then/catch/finally routed through RuntimeFn
  - Module binary import fixture added
  - TypeScript erased syntax patterns: tsc build_pass 1 → 668 (+66700%)
  - Name resolution already complete on master (7225708a8)
  - Total tests: 1780 → 2037 (+257), 0 regressions
- Architecture design review completed and captured as `arch-design-review-2026-05-11`
  - 25-section analysis covering coupling points, target architecture, refactoring methodology
  - 8 priority-ordered refactoring items defined (P1-P8)
  - Design principles and refactoring patterns saved to recursive memory
  - Run ready for Phase 2 (TO-BE Plan) per item

## Previous State (2026-05-10)

- `Object.seal` is now a registered builtin (item 173, commit 7a08bfa78).
  Runtime sets the SEALED flag on the object flags field.
  Only build_smoke verified; semantic_diff for seal/freeze descriptor
  interaction tracked as item 180.
- `semantic_diff_async_exception` enabled (item 172, commit 3cbf58915).
  Async exception handling (throw in async, try/catch around await)
  produces output matching Node.js.
- `build_smoke_module_augmentation` fixed to expect diagnostic (item 156, commit c28347041).
  Module augmentation correctly produces issue-5253 unsupported diagnostic.
- `promise_basic_matches_node_output` acceptance test registered (item 196).
  - Promise minimal substrate (constructor + static resolve) works end-to-end.
  - WAT runtime: `$promise_constructor`, `$promise_resolve`, `$promise_reject` all implemented and tested.
- Batch fixture registration for 11 open W4/W5 items (commit 5f26acf2d).
  - RED phase: fixtures + tests added for IDs 194, 195, 202, 205, 206, 211, 212, 213, 214, 215.
  - Tests in correct test files matching TRACKING.yaml acceptance criteria.
  - Items 211 (well-known symbols), 202 (live binding), 215 (async generator) already compile (GREEN at build level).
- Decorator syntax erasure implemented (ID 195, commit 3b94252b8).
  - `decorator_before_class_declaration` consumes `@expr` prefix, then parses decorated declaration.
  - Only statement-level class decorators; class-expression decorators still rejected.
