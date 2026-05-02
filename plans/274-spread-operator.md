# Plan: Issue 274 — Implement spread operator

## Objective

Implement spread operator (`...`) semantics for array literals, object literals, and function call arguments.

## Steps

1. Review existing spread lowering in `crates/ir/src/lowered/resolver_extra.rs`
2. Ensure IR/lowering handles spread in array/object literals and call args
3. Add/update Node/iwasm differential fixtures
4. Add IR regression coverage

## Validation

- `cargo nextest run -E 'test(spread) or test(node_diff)'`
- `mise run reference-coverage -- test262 --limit 2000`
