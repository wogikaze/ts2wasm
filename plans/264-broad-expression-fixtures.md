# Plan: Broad expression fixture coverage

## Problem

Expression-level coverage is ad-hoc. Many Expr AST variants lack dedicated fixtures.

## Approach

Create `fixtures/core-expressions/` with one `.ts` file per Expr variant, each exercising the expression via console.log so output can be validated. Register in the test catalog.

## Expr variants and fixture file mapping

| Variant | Fixture | Status |
|---------|---------|--------|
| Number | existing coverage | skip |
| BigInt | `bigint.ts` | new |
| String | existing coverage | skip |
| Bool | existing coverage | skip |
| Null | existing coverage | skip |
| Undefined | existing coverage | skip |
| Await | existing coverage | skip |
| Ident | existing coverage | skip |
| Unary | `unary.ts` | new |
| Binary | existing coverage | skip |
| Member | existing coverage | skip |
| OptionalMember | `optional-member.ts` | new |
| Call | existing coverage | skip |
| OptionalCall | `optional-call.ts` | new |
| Assign | existing coverage | skip |
| LogicalAssign | `logical-assign.ts` | new |
| LogicalPropertyAssign | `logical-property-assign.ts` | new |
| Array | existing coverage | skip |
| Object | existing coverage | skip |
| Index | existing coverage | skip |
| OptionalIndex | `optional-index.ts` | new |
| New | existing coverage | skip |
| TypeOf | existing coverage | skip |
| InstanceOf | existing coverage | skip |
| Ternary | existing coverage | skip |
| ArrowFn | existing coverage | skip |
| FunctionExpr | `function-expr.ts` | new |
| Spread | existing coverage | skip |
| PropertyAssign | existing coverage | skip |
| IndexAssign | `index-assign.ts` | new |
| This | existing coverage | skip |

## Changes

- New fixtures in `fixtures/core-expressions/`
- Register in `crates/cli/tests/common/m2_node_diff_fixture_tests.rs`

## Validation

```
cargo fmt --all --check
cargo nextest run
```
