# Refactoring Patterns

## Branch by Abstraction

Strategy for replacing String-typed runtime calls with typed enums:

1. Add new abstraction alongside existing (e.g., `RuntimeIntrinsic` enum)
2. Migrate partial usage
3. Deprecate old path
4. Architecture check bans new usage of old path
5. Complete migration
6. Delete old path

## Strangler Fig

Strategy for splitting giant functions like `lower_expr`:

1. Keep giant match as dispatcher
2. Extract one semantic domain branch at a time
   - Array literal → `array/literal.rs`
   - Class branch → `class/lower.rs`
   - Call branch → `call/lower.rs`
3. Dispatcher shrinks until it's just a routing table

## Characterization Tests

Before any refactoring, capture current output:

```
existing fixture output
link plan snapshot
lowered IR snapshot
```

Refactoring must not change this output.

## Mikado Method

Decompose large goals into dependency-ordered small tasks.

Example: "backend-wasm no longer depends on frontend"
→ Diagnostic → shared → Span → DiagCode → imports → cargo → CI check

## Validated<T> Wrapper

```rust
pub struct Validated<T> { inner: T }

impl Validated<LoweredProgram> {
    pub fn new(program: LoweredProgram) -> Result<Self, Vec<Diagnostic>> {
        validate_lowered(&program)?;
        Ok(Self { inner: program })
    }
    pub fn as_ref(&self) -> &LoweredProgram { &self.inner }
}
```

Backend API accepts only `&Validated<LoweredProgram>`.

## Status: CURRENT (2026-05-11)
