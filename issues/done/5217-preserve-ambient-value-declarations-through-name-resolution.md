---
id: 5217
title: "Preserve ambient value declarations through name resolution"
type: bug
area: ir/name-resolution
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Restore the current contract that ambient value declarations are preserved as metadata through name resolution but do not produce lowered runtime statements.

## Problem

Problem: full `cargo nextest run` now reaches two ambient value tests where `name_resolver` drops `AmbientValueDecl` during the second pass.

The resolver still collected ambient names for reference resolution, but it omitted the declaration metadata from the resolved statement stream. That violates issue 5161's contract and prevents builtin resolution from seeing `ResolvedStmt::AmbientValue`.

## Current failure

Reproduction:

```sh
cargo nextest run -p ts2wasm-ir resolves_ambient
```

Failure excerpt:

```text
resolves_ambient_const_shorthand_without_runtime_decl: resolved.len() left: 1, right: 2
resolves_ambient_value_decl_in_array_and_object_literals: resolved.len() left: 2, right: 3
```

## Desired final state

Ambient value declarations remain visible to name/builtin resolution, references to them resolve, and no runtime declaration statement is emitted for the ambient metadata.

## Scope

In scope:

- [x] Preserve `AmbientValueDecl` in the resolved statement stream without emitting runtime declarations after lowering.

Out of scope:

- Emitting runtime declarations for ambient values.
- Reverting issue 5161 or issue 5206 semantics.

## Affected paths

Expected:

- `crates/ir/src/name_resolver_tests.rs`
- `crates/ir/src/name_resolver.rs`

Do not touch:

- `crates/compiler/src/dump.rs`

## Acceptance criteria

- [x] `cargo nextest run -p ts2wasm-ir resolves_ambient` passes.
- [x] Full `cargo nextest run` no longer fails at the two ambient value count assertions.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir resolves_ambient
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
git diff --check
```

Impacted commands:

```sh
cargo nextest run
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Discovered while validating issue 5216.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- This commit.

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-ir resolves_ambient
result: pass
date: 2026-05-06

command: cargo nextest run -p ts2wasm-cli --test dump_cli dump_ast_unparse_erases_ambient_declarations
result: pass
date: 2026-05-06

command: cargo nextest run
result: pass, 1012 tests passed and 9 skipped
date: 2026-05-06
```

Remaining risks:

- none
