# Issues

This section tracks all project issues and their status.

## Summary

| Area | Total | Open | Resolved |
|---|---:|---:|---:|
| abi | 2 | 0 | 2 |
| backend | 5 | 1 | 4 |
| cli | 3 | 0 | 3 |
| compiler | 1 | 0 | 1 |
| docs | 2 | 0 | 2 |
| frontend | 191 | 127 | 64 |
| ir | 7 | 0 | 7 |
| issues | 4 | 0 | 4 |
| parser | 1 | 0 | 1 |
| reference | 2 | 2 | 0 |
| runtime | 63 | 10 | 53 |
| scripts | 2 | 0 | 2 |
| security | 1 | 0 | 1 |
| tests | 5 | 0 | 5 |
| wasi | 1 | 0 | 1 |
| total | 290 | 140 | 150 |

## Ready Queue

| ID | Title | Type | Area | Class | Priority | Depends on | Summary |
|---:|---|---|---|---|---|---|---|
| 225 | Implement eval and Annex B function declaration semantics | feature | frontend/semantics | implementation-ready | P3 |  | Direct `eval` and dynamic code evaluation are required JavaScript semantics; when wasm-only implementation is not suf... |
| 251 | Implement destructuring binding runtime semantics | feature | frontend/semantics | implementation-ready | P2 |  | Parsed destructuring binding patterns are accepted for AST/dump coverage, but name resolution, lowering, and runtime ... |
| 255 | Implement private class element runtime semantics | feature | runtime/semantics | implementation-ready | P2 |  | Issue 248 tokenizes `#name` and parses private fields, methods, getters, and setters. The first runtime slice now sup... |
| 260 | Implement BigInt arithmetic operators | feature | runtime/semantics | implementation-ready | P2 | 259 | Operators such as `1n + 2n` and `-1n` require BigInt-specific runtime helpers and must not reuse small-int `number` s... |
| 261 | Implement BigInt equality comparison and coercion boundaries | feature | runtime/semantics | implementation-ready | P2 | 259 | BigInt cannot share the current primitive equality/comparison helpers because Number/BigInt and String/BigInt coercio... |
| 262 | Implement BigInt builtins and string conversion | feature | runtime/builtins | implementation-ready | P2 | 259 | After BigInt values exist, builtin and conversion behavior still needs explicit runtime helpers and diagnostics so `B... |
