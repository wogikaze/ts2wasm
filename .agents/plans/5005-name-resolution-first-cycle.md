# Issue 5005: Name Resolution Coverage — First Cycle (Scope Analysis + DuplicateLocal)

## Context

Issue 5005 is a meta-issue covering ~428 child issues related to name resolution. Current name resolution (`crates/ir/src/name_resolver.rs`) already handles:
- Lexical scope chain with `Vec<HashMap<String, Option<Span>>>`
- Variable/function/class declaration binding
- Function hoisting via pre-pass
- Duplicate local detection in the same lexical scope
- Unresolved name reporting
- Label validation (break/continue)
- Class-as-value rejection (5011, already done)
- ~35 hardcoded allowed globals

Gaps include:
- `var` hoisting semantics (function-scoped vs block-scoped)
- Temporal Dead Zone
- Comprehensive test262 duplicate-local edge cases (issue 343, 66 cases)

## Plan

### Phase 1: Benchmark DuplicateLocal Coverage (issue 343)

Run test262 coverage to determine how many `DuplicateLocal` diagnostic cases remain.

### Phase 2: DuplicateLocal Edge Cases (if baseline shows gaps)

The name resolver already checks `declare_variable()` for re-declaration in the same scope. Edge cases to handle:
1. **Var across blocks** — `{ var x; } var x;` should be OK (var is function-scoped), but `{ let x; } let x;` should be OK (different block scope)
2. **Function declaration conflicts** — `function f() {} function f() {}` should produce DuplicateLocal
3. **Global scope vs function scope** — var at global vs let at global
4. **Catch parameter** — `try {} catch (e) { let e; }` — catch creates a new scope in ES2015+

### Phase 3: Fixture Tests

Add fixture tests for each edge case pattern that currently produces incorrect diagnostics or no diagnostics.

## Acceptance

- [x] Baseline measured: 66 reported DuplicateLocal cases
- [ ] Edge cases identified and implemented
- [ ] Fixture tests pass
- [ ] No regression in existing module/naming tests
- [ ] fmt + nextest pass

## Verification

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --limit 1000 --detail | grep DuplicateLocal
```
