# STATE.md

## Current State (2026-05-10)

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
