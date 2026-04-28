---
id: 059a
title: "Implement TypeScript satisfies and const assertion erasure"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: Parser syntax work needs the next small erasable TypeScript syntax slice instead of another broad parser epic selection.

## Summary

Extend the parser-only TypeScript erasure path to accept `expr satisfies Type` and `expr as const` without changing runtime semantics.

## Scope

In scope:

- [ ] Parse and erase `satisfies` type clauses in expression positions.
- [ ] Parse and erase `as const` assertions.
- [ ] Add parser and CLI dump/build coverage proving the runtime expression is preserved.

Out of scope:

- Type checking.
- Decorators.
- Private fields.
- General const-context inference.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/dump_cli.rs`
- `fixtures/basics-types/`

Do not touch:

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`

## Acceptance criteria

- [ ] Dump `--ast --unparse` erases `satisfies` and `as const` syntax.
- [ ] A fixture using both forms builds and runs with Node/iwasm differential parity.
- [ ] Adjacent relational expressions are not misparsed as type syntax.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli --test dump_cli
```

Impacted commands:

```sh
cargo run -q -p ts2wasm-cli -- dump --ast --unparse fixtures/basics-types/satisfies-const-erasure.ts
cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/satisfies-const-erasure.ts -o /tmp/ts2wasm-059a.wasm
iwasm /tmp/ts2wasm-059a.wasm
```

Not run:

- none

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
