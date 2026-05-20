# Frontend Syntax Ownership Contract

This document defines which syntax families belong to which compiler layer
(frontend, IR, runtime, backend), how ownership boundaries are enforced, and
what happens when a construct crosses a boundary.

## Classification categories

Every TypeScript/JavaScript construct falls into one of the following categories.
The category determines which layer owns the construct, how the compiler handles
it, and what diagnostic is raised when support is missing.

| Category | Owner | Compiler contract | Diagnostic |
|---|---|---|---|
| Parse-and-erase | Frontend | Parser accepts syntax; erasure pass removes type-only nodes before JS runtime lowering. No runtime IR or backend representation. | `UnsupportedTypeScriptSyntax` when missing, or silence when erasure works |
| Parse-and-preserve | Frontend + IR | Parser keeps structure for module graph or declaration shape. Runtime lowering receives only executable JS. | `UnsupportedModule` for module blockers; `UnsupportedTypeScriptSyntax` for unrepresentable forms |
| Parse-and-lower | Frontend + IR + Runtime | Frontend rewrites TS-to-JS transforms; IR lowers JS semantics; Runtime provides builtin helpers. | `UnsupportedTypeScriptSyntax` for missing TS transform; `UnsupportedRuntimeSubset` for missing JS runtime |
| Reject | Frontend | Parser recognizes form for a precise diagnostic; compiler never silently creates runtime bindings. | `UnsupportedTypeScriptSyntax` or `UnsupportedModule` |
| Deferred/runtime-bearing | Backend/Runtime | Lowered IR carries the construct to the backend emit layer. Runtime provides WAT/builtin implementation. | Handled by runtime capability matching |

## Syntax ownership table

### TypeScript-only syntax (frontend-owned, erased or rejected)

| Construct | Ownership | Category |
|---|---|---|
| Type annotations | Frontend (parser/lexer) | Parse-and-erase |
| Type aliases (`type T = ...`) | Frontend | Parse-and-erase |
| Interfaces | Frontend | Parse-and-erase |
| `as` assertions | Frontend | Parse-and-erase |
| Angle-bracket type assertions (non-JSX) | Frontend | Parse-and-erase |
| `satisfies` | Frontend | Parse-and-erase |
| Generic type parameters | Frontend | Parse-and-erase |
| Ambient declarations (`declare class`, `declare function`, etc.) | Frontend | Erase or reject |
| Ambient `declare module` / `declare namespace` | Frontend + Module | Reject (`UnsupportedModule`) |
| Type-only imports/exports | Frontend | Parse-and-erase |
| `const` assertions | Frontend | Parse-and-erase |
| Abstract classes/members (pattern only) | Frontend | Parse-and-erase |
| Overload signatures | Frontend | Parse-and-erase |

### TypeScript transform (frontend-owned, lower executable JS)

| Construct | Ownership | Category |
|---|---|---|
| Enums (runtime) | Frontend (transform) + Runtime | Parse-and-lower |
| Parameter properties | Frontend (transform) + Runtime | Parse-and-lower |
| Class accessors with bodies | Frontend (transform) + Runtime | Parse-and-lower |
| JSX (function-call emit) | Frontend (transform) + Runtime | Parse-and-lower |
| Decorators | Frontend (transform) + Runtime | Parse-and-lower |
| `namespace` with runtime body | Frontend (transform) + Runtime | Parse-and-lower |

### JavaScript runtime semantics (IR + Runtime owned)

| Construct | Ownership | Category |
|---|---|---|
| Control flow (if/while/for/switch/break/continue) | IR (lowered) + Backend | Deferred/runtime-bearing |
| Object literals and property access | IR + Runtime (object helpers) | Deferred/runtime-bearing |
| Array literals and indexed access | IR + Runtime (array helpers) | Deferred/runtime-bearing |
| Function/closure/allocation | IR + Runtime (closure helpers) | Deferred/runtime-bearing |
| Class semantics | IR + Runtime (class helpers) | Deferred/runtime-bearing |
| Builtin method calls | IR (routing) + Runtime (builtins) | Deferred/runtime-bearing |
| Exception handling (throw/try/catch) | IR + Runtime (completion records) | Deferred/runtime-bearing |
| Module import/export (executable) | IR + Runtime | Deferred/runtime-bearing |
| String/bigint operations | IR + Runtime | Deferred/runtime-bearing |

### Parser-only (frontend-owned, no runtime)

| Construct | Ownership | Category |
|---|---|---|
| Whitespace, comments, line endings | Frontend (lexer) | Parse-and-erase |
| Identifier resolution and scoping | Frontend + IR (name resolver) | Parse-and-preserve |
| Directive prologues | Frontend | Parse-and-preserve |

## Ownership boundary enforcement

### Frontend must not:
- Import from `ts2wasm_runtime_abi`, `ts2wasm_runtime_catalog`
- Reference `RuntimeFn`, `HostImport`, `CapabilityManifest` directly
- Emit raw WAT instructions or wasm imports
- Reference host import strings (`wasi_snapshot_preview1`, `host.*`)

### IR must not:
- Import from `ts2wasm_frontend` for its lowering/emit path
- Reference raw WAT instruction strings outside test modules
- Reference host import module strings directly (use `HostImport` type)

### Backend must not:
- Import from `ts2wasm_frontend` (except in test modules)
- Bypass `RuntimeFn` catalog by hardcoding runtime symbol strings

### Current legacy exceptions:
- `crates/ir` depends on `ts2wasm_frontend` (legacy, tracked by P7)
- `crates/backend-wasm` depends on `ts2wasm_frontend` (legacy, tracked by P4)

## HIR/MIR default-switch guard

The HIR/MIR pipeline (experimental) must not be the default build path. The
default build mode is `HirMirBuildMode::Disabled`. Opt-in is explicit via
`--experimental-hir-mir` or `--experimental-hir-mir-compat-fallback` CLI flags.

The `current-state.md` document must have an explicit "no-go"/"go" line for the
HIR/MIR default-switch status.
