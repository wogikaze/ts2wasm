# Ready Queue

Issues ready for implementation.

| ID | Title | Type | Area | Class | Priority | Depends on | Summary |
|---:|---|---|---|---|---|---|---|
| 225 | Implement eval and Annex B function declaration semantics | feature | frontend/semantics | implementation-ready | P3 |  | Direct `eval` and dynamic code evaluation are required JavaScript semantics; when wasm-only implementation is not suf... |
| 251 | Implement destructuring binding runtime semantics | feature | frontend/semantics | implementation-ready | P2 |  | Parsed destructuring binding patterns are accepted for AST/dump coverage, but name resolution, lowering, and runtime ... |
| 255 | Implement private class element runtime semantics | feature | runtime/semantics | implementation-ready | P2 |  | Issue 248 tokenizes `#name` and parses private fields, methods, getters, and setters. The first runtime slice now sup... |
| 260 | Implement BigInt arithmetic operators | feature | runtime/semantics | implementation-ready | P2 | 259 | Operators such as `1n + 2n` and `-1n` require BigInt-specific runtime helpers and must not reuse small-int `number` s... |
| 261 | Implement BigInt equality comparison and coercion boundaries | feature | runtime/semantics | implementation-ready | P2 | 259 | BigInt cannot share the current primitive equality/comparison helpers because Number/BigInt and String/BigInt coercio... |
| 262 | Implement BigInt builtins and string conversion | feature | runtime/builtins | implementation-ready | P2 | 259 | After BigInt values exist, builtin and conversion behavior still needs explicit runtime helpers and diagnostics so `B... |
