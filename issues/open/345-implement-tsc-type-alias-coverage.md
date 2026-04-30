---
id: 345
title: "Implement TypeScript type alias coverage for tsc suite (23 cases)"
type: feature
area: frontend/syntax
class: triage-needed
priority: P2
depends_on: [399]
blocks: []
created: 2026-04-30
updated: 2026-05-01
---

## Summary

TypeScript type alias declarations (`type Foo = ...`) are unsupported in 23 tsc suite test cases. The compiler currently fails on `type` keyword declarations used in the TypeScript test corpus.

## Problem

tsc coverage shows 23 cases blocked by type alias support (feature label: `type-alias`). The frontend needs to parse and erase type alias declarations when emitting wasm.

Problem: Child bucket of issue 399; 23 tsc suite cases fail due to missing type alias (`type X = ...`) parse/erase support, but implementation must wait for the TypeScript parse/erase/emit boundary contract.

## Current failure

```
mise run reference-coverage -- tsc --limit 6419
# Coverage matrix shows 23 type-alias failures
```

## Desired final state

The `type-alias` unsupported count in the tsc suite is reduced to 0. Type alias declarations are parsed and erased (no runtime emission), allowing the tsc test cases to compile.

## Scope

In scope:

- [ ] Implement parsing of `type` alias declarations
- [ ] Support generic type aliases
- [ ] Support type alias with complex type expressions (union, intersection, mapped types)
- [ ] Erase type aliases during IR lowering (no runtime emission)
- [ ] Add fixture tests

Out of scope:

- Runtime semantics of type aliases (they are compile-time only)
- Interface declarations
- Type alias re-exports with `export type`

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser.rs`
- `crates/ir/src/lowered.rs`
- `crates/ir/src/lowered/`
- `fixtures/`

Do not touch:

- `crates/runtime-abi/`

## Acceptance criteria

- [ ] Type alias unsupported count in tsc coverage decreases from 23
- [ ] Fixture tests cover basic, generic, and union type aliases
- [ ] Existing tsc suite cases that now pass are updated
- [ ] Docs/current-state/issues are synchronized when status or design changes

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- tsc --limit 6419
mise run update-coverage-matrix
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected
- [ ] updated: `docs/...`

Current state:

- [x] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

This issue is a child implementation bucket of issue 399. Do not start broad type-alias implementation until issue 399 defines the TypeScript parse/erase/emit boundary and confirms whether the tsc `type-alias` bucket should be handled by pure erasure, module-shape preservation, or a narrower child slice.

Boundary decision after issue 399: `type-alias` maps to category 1, parse and erase before runtime lowering. Representative failures that also require module-shape handling should be split out rather than widening this issue into module resolution or runtime semantics.

Type aliases should be purely erased during compilation if issue 399 confirms they have no runtime/module-shape effect for the selected cases. The main work is then in the parser to accept `type` keyword in declaration position and pass it through to the erasure pass.

## Progress evidence

2026-05-01 parent slice:

- Commit `9091f435` extends type alias erasure parsing to generic type
  parameter lists such as `type Box<T> = ...` and constrained/defaulted generic
  aliases such as `type MaybePair<T extends string | number, U = T> = ...`.
- Added parser, dump-unparse, and build fixture coverage for generic type alias
  erasure without runtime emission.

Validation result:

```text
cargo fmt --all --check: pass
cargo test -p ts2wasm-frontend parses_typescript_type_alias_declarations_as_erased_syntax -- --nocapture: pass
cargo test -p ts2wasm-cli dump_ast_unparse_erases_typescript_type_alias_declarations -- --nocapture: pass
cargo test -p ts2wasm-cli type_alias -- --nocapture: pass
```

Remaining:

- Not DONE. The required `mise run reference-coverage -- tsc --limit 6419`
  evidence has not yet been refreshed, so the tsc `type-alias` unsupported count
  is not verified as reduced to zero.

2026-05-01 parent coverage refresh:

- Full tsc limit coverage was refreshed after the generic alias erasure slice.
- Result: `type-alias` is still present with 41 unsupported cases in the
  `--limit 6419` window, so issue 345 remains open.
- The broader unsupported mix shows the next work is not only generic alias
  syntax; remaining cases overlap parser syntax, import/export, type-system, and
  declaration/module-shape boundaries.

Validation result:

```text
command: mise run reference-coverage -- tsc --limit 6419 --no-web-ui
result: pass; denominator=6419, build_pass=993, unsupported=5408, type-alias=41
date: 2026-05-01
```
