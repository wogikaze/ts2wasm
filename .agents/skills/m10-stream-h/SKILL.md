---
name: m10-stream-h
description: Use when implementing M10 Stream H performance and type-driven optimization foundations, including TypeScript type extraction, simple inference, fast paths, inline caching, and monomorphic optimizations in ts2wasm.
---

# Stream H: Performance & Type-Driven Optimization (M9 foundations)

## Goal
Implement TypeScript type information integration and type-driven fast paths to accelerate common patterns.
This provides M9 foundations: "TypeScript 型情報を使った primitive fast path が入る"

## Scope (1-2 hour window, foundational only)

Implement:
1. **Type information extraction** from TypeScript source (without full AST)
2. **Type inference** for simple patterns (literals, returns, assignments)
3. **Fast path generation** for typed arithmetic, string ops, array access
4. **Inline caching** for method calls (simple shape check)
5. **Monomorphic optimization** for common call sites

Not yet: Full TypeScript type checking, type-based JIT, generics specialization, whole-program analysis.

## Implementation strategy

### Phase 1: Type information model (15 min)

Define simple type representation:
```rust
#[derive(Debug, Clone)]
pub enum SimpleType {
    Number,
    String,
    Boolean,
    Array(Box<SimpleType>),
    Object,  // Generic object (no property types yet)
    Any,
    Unknown,
}

pub struct TypeInfo {
    expr: Expr,
    inferred_type: SimpleType,
    confidence: u8,  // 0-100: how sure are we?
}
```

### Phase 2: Type inference pass (20 min)

Implement basic type inference:
```rust
fn infer_type(expr: &Expr) -> SimpleType {
    match expr {
        Expr::Number(_) => SimpleType::Number,
        Expr::String(_) => SimpleType::String,
        Expr::Bool(_) => SimpleType::Boolean,
        Expr::BinaryOp(BinaryOp::Add, left, right) => {
            // If both numbers, result is number
            // If one string, result is string
            infer_binop_type(infer_type(left), infer_type(right))
        }
        Expr::ArrayLiteral(items) => {
            // Infer element type from items
            let elem_type = infer_type(&items[0]);
            SimpleType::Array(Box::new(elem_type))
        }
        // ... more patterns
    }
}
```

### Phase 3: Fast path codegen (20 min)

For typed operations, emit optimized WAT:
```wasm
;; Generic addition (slow path)
(func $add_generic (param $a i32) (param $b i32) (result i32)
  ;; Check types, dispatch to appropriate operation
  ;; ... slow ...
)

;; Typed fast path (both known to be numbers at compile time)
(func $add_numbers_fastpath (param $a i32) (param $b i32) (result i32)
  ;; Direct arithmetic, skip type checks
  (i32.or
    (i32.shl
      (i32.add
        (i32.shr_s (local.get $a) (i32.const 3))
        (i32.shr_s (local.get $b) (i32.const 3)))
      (i32.const 3))
    (i32.const 4))  ;; number tag
)
```

Decision logic in emit_expr:
```rust
if let Some(type_info) = type_db.get_type(&expr) {
    if type_info.confidence > 80 {
        emit_typed_fastpath(type_info.inferred_type, ...);
    } else {
        emit_generic_path(...);
    }
}
```

### Phase 4: Inline caching (15 min)

For method calls, cache the method location:
```wasm
;; Inline cache: stores expected object shape
(global $ic_shape_cache i32 (i32.const 0))
(global $ic_method_ref i32 (i32.const 0))

(func $call_method_with_ic (param $obj i32) (param $method_name i32) (result i32)
  ;; Check if object shape matches cached shape
  (if (i32.eq (call $object_shape (local.get $obj)) (global.get $ic_shape_cache))
    (then
      ;; Fast path: use cached method
      (call (global.get $ic_method_ref) (local.get $obj))
    )
    (else
      ;; Slow path: lookup method, update cache
      ;; ...
    )
  )
)
```

### Phase 5: Monomorphic call tracking (10 min)

Track call sites that always receive the same type:
```rust
struct CallSiteProfile {
    expr_id: ExprId,
    observed_types: HashMap<SimpleType, usize>,  // type → count
    total_calls: usize,
}

// During lowering, note when a function always receives int args
if arg_types.all(|t| t == SimpleType::Number) {
    mark_monomorphic(call_site_id, SimpleType::Number);
}
```

Generate specialized versions:
```wasm
;; Original function (generic)
(func $compute (param $x i32) (result i32) ...)

;; Specialized for number argument
(func $compute_number_specialization (param $x i32) (result i32) ...)

;; Dispatch wraps them:
(func $compute (param $x i32) (result i32)
  (if (i32.eq (i32.and (local.get $x) (i32.const 7)) (i32.const 4))  ;; is number?
    (then (call $compute_number_specialization (local.get $x)))
    (else (call $compute_generic (local.get $x)))
  )
)
```

### Phase 6: Integration with TypeScript types (10 min, optional)

If TypeScript annotations present, use them:
```typescript
function add(x: number, y: number): number {
    return x + y;
}
```

Extract type annotations → inject into type database with 100% confidence.

For now, focus on inference from usage; explicit annotations are future work.

### Phase 7: Tests (10 min)

Fixtures:
1. `typed-arithmetic.ts`: number+number → number (should use fast path)
2. `typed-array-access.ts`: array[number] with known array type
3. `typed-method-call.ts`: calling same method on same object type repeatedly
4. `monomorphic-function.ts`: function always called with same argument type
5. `polymorphic-function.ts`: function called with mixed types (slow path fallback)

Compare:
- Wasm size (fast path should be smaller)
- Execution time (if measurable in small fixture; defer to Stream G)

## Output

**Commits**:
1. `ir: add TypeInfo and type inference to lowering`
2. `backend: implement type-driven fast path codegen`
3. `backend: add inline caching for method calls`
4. `backend: add monomorphic call specialization`
5. `backend: integrate TypeScript type annotations into type database (if present)`
6. `tests: add typed optimization integration tests`

**Files modified**:
- `crates/cli/src/ir/lowered.rs` (add TypeInfo)
- `crates/cli/src/backend/expr_emit.rs` (add fast path dispatch)
- `crates/cli/src/backend/runtime_builder.rs` (emit IC stubs)
- `crates/cli/src/lib.rs` (type inference pass)

**Tests added**:
- `crates/cli/tests/m9_typed_optimization.rs`
- Fixture files: `fixtures/m9/typed-*.ts`

**DiagCode impact**:
- No new UnsupportedSyntax (optimization is transparent)
- Type inference improves efficiency without changing semantics

**Coverage matrix delta**:
- No change in pass/fail/unsupported counts (optimization doesn't change what compiles)
- Wasm size may decrease (fast paths are more compact)
- Execution time improves (if baseline tracking in Stream G)

## Validation before commit

```bash
cargo fmt --all --check
cargo test -q
cargo test -q --test m9_typed_optimization
# Compare wasm sizes (optimized vs non-optimized)
./target/debug/ts2wasm-cli build fixtures/m9/typed-arithmetic.ts -o /tmp/opt.wasm
ls -la /tmp/opt.wasm
# Compare with non-typed version
./target/debug/ts2wasm-cli build fixtures/m9/untyped-arithmetic.ts -o /tmp/no-opt.wasm
# Should see difference in output size
```

## Gatekeeper checklist

✓ Type inference conservative (only high-confidence types used for fast path)
✓ Fallback to generic path if type check fails at runtime
✓ Inline cache invalidation works (shape mismatch triggers slow path)
✓ Monomorphic specialization doesn't break polymorphic calls
✓ TypeScript annotation extraction doesn't alter semantics
✓ Fast paths produce correct results (compare with generic path)
✓ No observable performance regression if type info is absent

## Design decisions

1. **Type inference scope**: Single-pass, local only (not inter-procedural)
2. **Confidence threshold**: 80% required for fast path (empirical; can tune)
3. **Inline cache size**: Single-entry (first call sets cache)
4. **Monomorphic threshold**: All observed calls monomorphic; one polymorphic site = disable optimization
5. **TypeScript integration**: Best-effort (ignore unsupported syntax; fall back to inference)

## M9 gate prerequisites

Completion of Stream H enables:
- ✓ Typed fast paths reduce wasm code size
- ✓ Typed arithmetic avoids runtime dispatch
- ✓ Inline caches improve repeated method calls
- ✓ Performance dashboards show measured speedups

M9 success means:
- Typed fixture runs 10-20% faster than untyped equivalent
- Wasm size reduced by ~5-15% through fast paths
- TypeScript type information reduces runtime checks

## Future enhancements (not in scope)

- Profile-guided optimization (collect runtime data, regenerate)
- Inter-procedural type inference
- Escape analysis (stack allocation for short-lived objects)
- Inlining based on type information
- Shape specialization (different fast paths for different object layouts)

## References

- Type information sources: `docs/05-compatibility-and-semantics.md`
- Existing arithmetic: `crates/cli/src/backend/runtime_builder.rs` ($add, $sub, etc.)
- Fast path pattern: integer arithmetic + tag operation (already used)
