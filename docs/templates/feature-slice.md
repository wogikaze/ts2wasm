# Vertical-Slice Feature Template

Use this checklist when adding a new JS builtin, syntax feature, or runtime function.
Each row is a required layer unless marked optional.

## Syntax impact

- [ ] Does the feature add new syntax? If yes:
  - [ ] AST node or ResolvedExpr variant added
  - [ ] Parser handles the syntax
  - [ ] Name resolver handles any new scoping
  - [ ] Dump implementation covers the new node
- [ ] Is the feature purely runtime (no syntax change)? If yes, skip syntax layers.

## Builtin resolution

- [ ] BuiltinId / BuiltinResolver entry added
- [ ] New ResolvedExpr variant or existing variant reused
- [ ] LoweredExpr or RuntimeIntrinsic variant for the new op

## Resolver lowering (IR)

- [ ] Resolver domain module updated (e.g., array.rs, object.rs, string.rs)
- [ ] LoweredExpr / LoweredStmt produced for the operation
- [ ] Object kernel or completion record used where applicable (instead of inline RuntimeCall)
- [ ] HIR/MIR boundaries respected (future-proof)

## Runtime catalog

- [ ] RuntimeFn variant added to `ts2wasm_runtime_catalog`
- [ ] RuntimeSpec entry (input/output types, runtime dependencies)
- [ ] Runtime domain assignment (core/array/string/object/date/host/promise/bigint)
- [ ] Import requirements (WASI shim, Node shim, or none)
- [ ] Capability marker (stdout, filesystem, random, etc.)

## Backend WAT emission

- [ ] Runtime function WAT implementation in the correct domain file
- [ ] Domain dispatch arm in runtime_builder.rs
- [ ] RuntimeLinkPlan picks up the new function automatically

## Link plan and manifest

- [ ] RuntimeLinkPlan includes the function when the feature is used
- [ ] RuntimeLinkPlan manifests correctly: `build_runtime_link_plan(program)` picks it up
- [ ] Summary snapshot test: `emit_link_plan_snapshot_json` produces expected JSON
- [ ] CapabilityManifest reflects the correct capabilities
- [ ] No unnecessary imports leaked to the manifest
- [ ] Emission order constraint if any

## Differential test

- [ ] Fixture added under `fixtures/` (minimal positive case)
- [ ] Node output captured (or expected output defined)
- [ ] `m2_node_diff` test entry added to fixture list
- [ ] Differential pass confirmed: `cargo nextest run -p ts2wasm-cli --test m2_node_diff <filter>`

## Negative tests

- [ ] Compiler gracefully rejects invalid syntax related to this feature
- [ ] Unsupported variants produce `[UnsupportedSyntax]` or `[UnsupportedBuiltin]` diagnostic (not a panic)
- [ ] Test added for edge case: missing arguments, wrong types, or null/undefined inputs
- [ ] Regression test: existing features continue to work after new feature is added
- [ ] If applicable: malformed input produces a diagnostic, not a crash
- [ ] Test added under `fixtures/test-infrastructure/unsupported-fixture.ts` or equivalent

## Architecture checks

- [ ] `mise run check architecture` passes (no new violations)
- [ ] Function lengths remain below 300 lines (error) / 200 lines (warning)
- [ ] No raw runtime symbols outside catalog
- [ ] No hardcoded WASI import strings outside catalog
- [ ] RuntimeFn spec/manifest/emission entries complete

## Coverage and docs

- [ ] build_smoke test added (compilation check)
- [ ] Coverage dashboard updated (if applicable to test262)
- [ ] `docs/current-state.md` reflects the new capability
- [ ] `issues/` entry closed with commit evidence
