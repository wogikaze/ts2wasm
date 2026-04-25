# Stream C: Control Flow & Statement Extensions

## Goal
Implement lowering and runtime support for try-catch-finally, switch, for/for-in/for-of, do-while, and break/continue.

## Scope (1-2 hour window)

Leverage Stream A parser AST; implement lowering for:
1. **try-catch-finally** → exception unwinding (WAT structured)
2. **switch-case** → pattern matching (WAT blocks/tables)
3. **for/for-in/for-of** → loop structures (WAT block/loop)
4. **do-while** → loop post-test (WAT loop)
5. **break/continue** → loop exit/restart (WAT br labels)
6. **throw** → exception signaling (error value propagation)

## Implementation strategy

### Phase 1: Exception model (15 min)

Design exception representation in WASM runtime:
- Option A: Special error value (tagged as exception, not JS value)
- Option B: Error parameter passed through call stack
- Option C: Local exception handler register

Recommendation: **Option A** (simplest for WASM stack machine)
- Define `ValueTag::EXCEPTION` (or use special high bit pattern)
- Exception values propagate up function calls
- `try` block catches exceptions, clears exception flag, restores normal value flow
- `finally` always runs, preserves exception state if not cleared

### Phase 2: Lower IR extension (10 min)

Add to `LoweredIR`:
```rust
enum Stmt {
    TryCatch {
        try_body: Vec<Stmt>,
        catch_var: Option<String>,  // exception bound to variable
        catch_body: Vec<Stmt>,
        finally_body: Option<Vec<Stmt>>,
    },
    Switch {
        expr: Expr,
        cases: Vec<(Option<Expr>, Vec<Stmt>)>,  // None = default
    },
    Break,
    Continue,
    Throw { expr: Expr },
}
```

### Phase 3: WAT Emission (45 min)

#### Try-Catch-Finally

Pattern:
```wasm
(block $try_exit
  (block $catch_entry
    ;; TRY BODY
    (if (call $is_exception (local.get $val))
      (then
        ;; value is exception; jump to catch
        (br $catch_entry)))
    ;; ... normal flow ...
    (br $try_exit)  ;; skip catch
  )
  ;; CATCH BODY
  (local.set $exception_var (local.get $val))
  ;; catch statements
)
;; FINALLY BODY (always executes)
;; ...
```

Emit:
```rust
fn emit_try_catch(&self, stmt: &TryCatch, wat: &mut String) {
    // Generate block labels for try/catch/finally
    // Emit try body with exception check
    // Emit catch binding + body
    // Emit finally
    // Restore value on exit
}
```

#### Switch-Case

Pattern:
```wasm
(block $switch_exit
  (block $case_1 (block $case_2 ... (block $default
    ;; Jump to matching case or default
    (br_table $case_1 $case_2 ... $default (call $compute_case_index))
  )))
  ;; CASE 1 BODY
  (br $switch_exit)  ;; break prevents fall-through (unless no break)
  ;; CASE 2 BODY
  ...
)
```

Emit:
```rust
fn emit_switch(&self, stmt: &Switch, wat: &mut String) {
    // Generate case index from expression
    // br_table to cases
    // emit case bodies
    // handle break vs fall-through
}
```

#### For-Loop & For-In/For-Of

Traditional for:
```wasm
(block $for_exit
  (loop $for_loop
    ;; initialization (once)
    ;; condition check
    (br_if $for_exit (i32.eqz (condition)))
    ;; body
    ;; update
    (br $for_loop)
  )
)
```

For-in (iterate object keys):
```wasm
;; Load object keys array
;; Loop over indices
;; For each index, bind key and execute body
```

For-of (iterate array/iterable):
```wasm
;; Load array/iterable
;; Similar to for-in but bind value instead of index
```

#### Do-While

```wasm
(block $do_exit
  (loop $do_loop
    ;; body
    ;; condition check
    (br_if $do_loop (condition))
  )
)
```

#### Break/Continue

- **Break**: `(br $loop_exit)` from inner loop context
- **Continue**: `(br $loop_continue)` to skip to update/condition

Label tracking during emission:
```rust
struct LoopContext {
    continue_label: String,
    exit_label: String,
}

// Stack of active loops
let mut loop_stack: Vec<LoopContext> = ...;
```

### Phase 4: Exception propagation (15 min)

Update all emit functions to handle exceptions:
- After each statement, check if result is exception
- If exception, propagate upward
- Only catch blocks and finally blocks can intercept

Pattern in emit_expr:
```rust
let val = emit_expr(...);
// Check if exception
if is_exception(val) {
    // Propagate to caller
    return val;
}
// Normal processing
```

### Phase 5: Tests (15 min)

Fixtures:
1. `try-basic.ts`: try-catch with exception
2. `try-finally.ts`: finally always runs
3. `switch-case.ts`: switch with fall-through and break
4. `for-loop.ts`: traditional for, for-in, for-of
5. `do-while.ts`: post-test loop
6. `break-continue.ts`: loop control
7. `nested-loops.ts`: break from nested loop
8. `exception-propagation.ts`: exception flows through function calls

All fixtures compare output with Node.js.

## Output

**Commits**:
1. `ir: add try-catch-finally, switch, break, continue, throw to LoweredIR`
2. `backend: add exception model to WAT runtime`
3. `backend: emit try-catch-finally runtime support`
4. `backend: emit switch-case-default statement`
5. `backend: emit for/for-in/for-of/do-while loops`
6. `backend: emit break/continue labels`
7. `backend: add exception propagation to expression emission`
8. `tests: add control flow integration tests (fixtures m6)`

**Tests added**:
- `crates/cli/tests/m6_control_flow.rs`
- Fixture files: `fixtures/m6/try-*.ts`, `fixtures/m6/switch-*.ts`, `fixtures/m6/loop-*.ts`, `fixtures/m6/exception-*.ts`

**DiagCode impact**:
- try/catch/finally/switch/break/continue no longer parse as UnsupportedSyntax
- Expect new "not yet runtime implemented" diagnostics if lowering stubs exist

**Coverage matrix delta**:
- test262 category: "control flow" should show execution progress
- `unsupported` decreases for basic control structure tests
- `pass` increases for test262 files using only basic control flow

## Validation before commit

```bash
cargo fmt --all --check
cargo test -q
cargo test -q --test m6_control_flow
# Verify exception handling
./target/debug/ts2wasm-cli build fixtures/m6/try-basic.ts -o /tmp/t.wasm
iwasm /tmp/t.wasm
# Compare with Node
node fixtures/m6/try-basic.ts
```

## Gatekeeper checklist

✓ Exception values propagate through calls
✓ finally block executes before exit
✓ break/continue use correct labels
✓ switch fall-through works (no automatic break)
✓ for-in iterates object keys (insertion order)
✓ for-of iterates array values
✓ Nested loops have distinct labels
✓ No string literals for WAT labels (generate unique names)
✓ All test fixtures have expected outputs

## Design decisions

1. **Exception representation**: Special ValueTag (simplest, avoids try-throw-return ambiguity)
2. **Switch fall-through**: Explicit (no automatic break between cases) → requires explicit break in fixtures
3. **For-in**: Iterates object's own properties (no prototype chain initially)
4. **For-of**: Assumes array-like (index 0...length) or explicit iterator protocol (deferred)
5. **Break scope**: Breaks innermost loop; labeled break deferred to Stream A lookahead enhancement

## References

- Current LoweredIR: `crates/cli/src/ir/lowered.rs`
- Current WAT emitter: `crates/cli/src/backend/runtime_builder.rs`
- Existing loop test: `fixtures/m2/while.ts` (reference implementation)
