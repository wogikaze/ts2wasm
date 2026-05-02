# Plan: Issue 5004 — Meta: Runtime Builtins Coverage (test262)

## Objective

Assess and improve runtime builtin coverage for test262 suite.

## Steps

1. Review current runtime builtin coverage gaps from test262 reference results
2. Identify highest-impact missing builtins (beyond Math/console/Number/Boolean already covered by 341)
3. File child issues for each builtin family
4. Implement the simplest builtin fix

## Validation

- `cargo nextest run -E 'test(node_diff)'`
