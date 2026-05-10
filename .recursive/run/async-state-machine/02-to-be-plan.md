# Phase 2: TO-BE Plan

## Requirement Mapping

### Item 170 — State-machine transformation (iwasm/CoreWasm target)

**Acceptance**: `cargo nextest run -p ts2wasm-cli --test m12_async_await semantic_diff_async_return`
**Goal**: Add `is_async` flag to Stmt::Function, propagate through IR, emit state-machine skeleton in backend so async return values are wrapped in a polling-compatible frame

### Item 171 — Runtime polling ABI (task_poll/task_result)

**Acceptance**: `cargo nextest run -p ts2wasm-cli --test m12_async_await semantic_diff_await_sequence`
**Goal**: Define RuntimeFn::TaskPoll, RuntimeFn::TaskResult, RuntimeFn::TaskDrop with WAT implementations

### Item 172 — JS Promise wrapper for wasm+js target

**Acceptance**: `cargo nextest run -p ts2wasm-cli --test m12_async_await semantic_diff_async_exception`
**Goal**: Implement JS host shim that drives the state machine via queueMicrotask

## Plan Drift Check

All three items are sequential dependencies: 170 → 171 → 172. Item 170 enables the state machine, item 171 adds runtime polling support, item 172 adds JS Promise integration.

## Implementation Plan

### Step 1: Propagate `is_async` through AST → Resolved → Lowered IR

**Files to modify**:

1. **`crates/frontend/src/ast.rs`** — `Stmt::Function { ... }` → add `is_async: bool`
2. **`crates/frontend/src/parser/statements_general.rs`** — `async_function_statement()` → set `is_async: true` on `Stmt::Function`
3. **`crates/ir/src/builtin_resolved.rs`** — `ResolvedStmt::Function { ... }` → add `is_async: bool`
4. **`crates/ir/src/name_resolver.rs`** — pass `is_async` through when creating `ResolvedStmt::Function`
5. **`crates/ir/src/lowered/types.rs`** — `LoweredFunction { ... }` → add `is_async: bool`
6. **`crates/ir/src/lowered/resolver.rs`** — pass `is_async` through when lowering to `LoweredFunction`
7. **`crates/ir/src/lowered/validate.rs`** — ensure `is_async` is recognized in validation (no extra validation needed initially)
8. **`crates/ir/src/semantic.rs`** — pass `is_async` through HIR lowering

**All existing `Stmt::Function` and `LoweredFunction` construction sites** must add the new field:
- `crates/backend-wasm/src/lib.rs` (test code with LoweredFunction constructors)
- `crates/cli/tests/` (test code)
- `crates/frontend/src/parser/` (other function parsing paths)

**Test approach**: Add `build_smoke_async_return` remains passing (it already passes by virtue of `is_async` being correctly propagated — the async flag reaching the backend triggers backend changes below).

### Step 2: State-machine backend emission

**Files to modify**:

1. **`crates/backend-wasm/src/emitter.rs`** — `emit_functions()`:
   - For async functions: frame lifting (locals → heap frame), emit wrapper function with br_table resume points
   - Frame structure: [state_id, local_0, local_1, ...] on heap
   - Function entry: load state, br_table to resume point
   - Each await point: save state, save live locals, return pending status
   - On completion: return resolved value

2. **`crates/backend-wasm/src/expr_emit.rs`** — `emit_expr`:
    - `PromiseGetValue`: emit state-machine yield at this point
    - Save current state ID, save all live locals to frame, return STATUS_PENDING
    - On resume, restore locals and continue

**Simplified approach for initial implementation**:
- The state-machine wrapper wraps the function body in a frame-polling loop
- Frame: `{ state: i32, return_value: i64, local_0: i64, local_1: i64, ... }`
- Initial state 0: execute body, each `await` increments state
- Final: store result in `return_value` slot and return STATUS_READY

### Step 3: Runtime polling ABI (item 171)

**Files to modify**:

1. **`crates/backend-wasm/src/runtime_fn.rs`** — Add `RuntimeFn` variants:
    - `TaskPoll` — polls a task by frame pointer
    - `TaskResult` — extracts result from a completed task
    - `TaskDrop` — deallocates a task frame
    - Add dependency constants

2. **`crates/backend-wasm/src/runtime_fn_impl.rs`** — Add `RuntimeSpec` entries:
    - symbols, deps, imports, capabilities, runtime_strings, result

3. **`crates/backend-wasm/src/runtime_builder.rs`** — Add WAT emit functions:
    - `emit_task_poll`: read frame.state, check completion
    - `emit_task_result`: read frame.return_value
    - `emit_task_drop`: free frame memory

4. **`crates/backend-wasm/src/runtime_async.rs`** (new file) — WAT implementations:
    - `$task_poll(frame_ptr) -> i32`: reads `frame[0]` (state). If state == DONE (max state), returns 1 (READY), else 0 (PENDING)
    - `$task_result(frame_ptr) -> i64`: reads `frame[1]` (return_value)
    - `$task_drop(frame_ptr)`: calls `$free` on the frame

5. **`crates/backend-wasm/src/runtime_link_plan.rs`** — Wire TaskPoll/TaskResult/TaskDrop into the link plan

### Step 4: JS Promise wrapper (item 172)

**Files to modify**:

1. **`crates/backend-wasm/src/lib.rs`** — Add JS shim emission for async targets:
    - For `--target wasm+js`: emit a JS wrapper that creates a Promise
    - The wrapper: `queueMicrotask(() => { poll state machine; if READY resolve(result); else queueMicrotask again })`

2. **Update fixture tests**: Un-ignore semantic_diff tests with appropriate JS shim runner

## Plan Structure for TDD

Each step follows RED-GREEN-REFACTOR:

- **Step 1** (is_async propagation):
  - RED: Write test `build_smoke_async_return` already exists as build_smoke (passes)
  - Write a new test that verifies `is_async` flag is set in the compiled program
  - GREEN: Add `is_async` to all IR types

- **Step 2** (state-machine backend):
  - RED: Un-ignore `semantic_diff_async_return` and confirm it fails
  - GREEN: Implement state-machine emission

- **Step 3** (polling ABI):
  - RED: Un-ignore `semantic_diff_await_sequence` and confirm it fails
  - GREEN: Add RuntimeFn variants and WAT implementations

- **Step 4** (JS Promise wrapper):
  - RED: Un-ignore `semantic_diff_async_exception` and confirm it fails
  - GREEN: Add JS shim emission

## Files to Touch (complete list)

```
crates/frontend/src/ast.rs                       -- Stmt::Function add is_async
crates/frontend/src/parser/statements_general.rs  -- async_function_statement set is_async
crates/frontend/src/parser/expressions_main.rs    -- await expression handling (no change)
crates/ir/src/builtin_resolved.rs                 -- ResolvedStmt::Function add is_async
crates/ir/src/name_resolver.rs                    -- pass is_async through
crates/ir/src/lowered/types.rs                    -- LoweredFunction add is_async
crates/ir/src/lowered/resolver.rs                 -- pass is_async through
crates/ir/src/lowered/validate.rs                 -- recognize is_async
crates/ir/src/semantic.rs                         -- pass is_async through
crates/backend-wasm/src/runtime_fn.rs             -- TaskPoll/TaskResult/TaskDrop
crates/backend-wasm/src/runtime_fn_impl.rs        -- RuntimeSpec entries
crates/backend-wasm/src/runtime_builder.rs        -- WAT emit functions
crates/backend-wasm/src/runtime_async.rs           -- NEW: WAT polling implementations
crates/backend-wasm/src/emitter.rs                -- state-machine function emission
crates/backend-wasm/src/expr_emit.rs              -- PromiseGetValue as state-machine yield
crates/backend-wasm/src/runtime_link_plan.rs       -- wire new RuntimeFn
crates/backend-wasm/src/lib.rs                    -- register new module, JS shim
crates/cli/tests/m12_async_await.rs               -- un-ignore semantic diff tests
```

## Acceptance Commands

```bash
# Item 170
cargo nextest run -p ts2wasm-cli --test m12_async_await semantic_diff_async_return

# Item 171
cargo nextest run -p ts2wasm-cli --test m12_async_await semantic_diff_await_sequence

# Item 172
cargo nextest run -p ts2wasm-cli --test m12_async_await semantic_diff_async_exception
```

## Requirement Completion Status

To be filled during implementation.

## Non-Goals

- No JSPI (JavaScript Promise Integration) support
- No full Promise/A+ specification compliance
- No WASI P3 / Component Model async lowering
- No full async iteration (for-await-of implementation beyond passthrough)
