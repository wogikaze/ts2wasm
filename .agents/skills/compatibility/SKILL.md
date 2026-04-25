---
name: compatibility
description: Use when changing TypeScript/ECMAScript compatibility, runtime semantics, WASI capability lowering, Node.js host fallback, or AssemblyScript boundary rules in this compiler.
---

# Language compatibility

The input language is TypeScript plus ECMAScript runtime semantics. Do not require AssemblyScript-only syntax, primitive types, intrinsics, or standard library behavior from users.

## Mise: run before you finish (required for semantic changes)

**A compatibility change is not “done” until the relevant test gates are green; run the commands below and record outcomes.** Without `mise`, use `scripts/manager` with the same name. First time: `mise trust` ([docs](https://mise.jdx.dev/cli/trust.html)).

```bash
mise run fmt
mise run clippy
mise run nextest
```

- If test262 or differential work is in scope: `mise run test262` (and pipe through `mise run test-differential-reporter` / `mise run test-regression-gate` with the project’s file conventions; see `scripts` headers)
- If reference-suite coverage is relevant: `mise run reference-coverage` (as documented for that script)

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
