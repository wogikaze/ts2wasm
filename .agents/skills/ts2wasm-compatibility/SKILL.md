---
name: ts2wasm-compatibility
description: Use when changing TypeScript/ECMAScript compatibility, runtime semantics, WASI capability lowering, Node.js host fallback, or AssemblyScript boundary rules in ts2wasm.
---

# ts2wasm Compatibility Workflow

The input language is TypeScript plus ECMAScript runtime semantics. Do not require AssemblyScript-only syntax, primitive types, intrinsics, or standard library behavior from users.

## Decision Order

1. Check whether the behavior is TypeScript syntax, ECMAScript runtime semantics, host capability, or optimization.
2. Preserve JavaScript observable semantics by default.
3. Use TypeScript types only as optimization hints unless guards prove the fast path safe.
4. Prefer WASI lowering for portable APIs.
5. Use Node.js host fallback only when WASI/runtime cannot represent the behavior.

## Capability Rules

- `process.argv` maps to WASI args when possible.
- `process.env` maps to WASI environ and a runtime facade when possible.
- Node.js fallback must be listed as `host.<domain>.<function>` in the capability manifest.
- Host imports must be function-level, not whole-module imports.

## Test Expectations

Every compatibility change needs a classified test status: `pass`, `fail`, `unsupported`, `blocked`, or `skip-with-reason`.
