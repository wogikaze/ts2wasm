# Issue 350 Implementation Plan

## Background

Private slot allocation is class-local: each class's fields start at slot 0.
Derived classes need accumulated slot counts (self + ancestors) so parent
private fields don't overlap with child private fields in memory.

## Approach

1. Add `ancestor_private_slot_count()` that walks the extends chain
2. Fix `private_slot_count()` to return self + ancestors
3. Fix slot lookup to add ancestor offset for derived class fields
4. Add fixture: `fixtures/core-semantics/derived-class-private-field.ts`
5. Run cargo nextest to verify

## Files

- `crates/ir/src/lowered/resolver_extra.rs`: slot count + base offset
- `crates/ir/src/lowered/resolver_expr.rs`: pass total count to New expr
- `fixtures/core-semantics/derived-class-private-field.ts`: test
