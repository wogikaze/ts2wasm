---
id: 059a
title: "Implement TypeScript satisfies and const assertion erasure"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
status: done
---

## Summary

Implement a narrow parser-only TypeScript erasure slice for `satisfies` expressions and const assertions.

## Problem

The parser must accept runtime-erased TypeScript syntax used by modern TypeScript source without forwarding type-only operands into AST, lowering, or wasm emission.

## Desired final state

`expr satisfies Type`, `expr as const`, and `<const>expr` parse as the runtime expression. `dump --ast --unparse` and build output contain only executable JavaScript semantics.

## Scope

In scope:

- [x] Preserve existing `satisfies` erasure coverage.
- [x] Preserve existing `as const` erasure through the `as` assertion path.
- [x] Add `<const>expr` const assertion erasure in unary expression position.
- [x] Add combined regression fixture and CLI coverage.

Out of scope:

- [x] Full TypeScript type checking.
- [x] General angle-bracket type assertions beyond the const assertion form.
- [x] Decorators, private fields, and broader issue 059 parser-syntax reduction.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/dump_cli.rs`
- `fixtures/basics-types/`

Do not touch:

- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `reference/`

## Acceptance criteria

- [x] Parser accepts `satisfies` expressions as erased syntax.
- [x] Parser accepts `as const` and `<const>expr` const assertions as erased syntax.
- [x] `dump --ast --unparse` erases the TypeScript-only syntax.
- [x] Build accepts the combined regression fixture and emitted wasm runs under `iwasm`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli --test dump_cli
cargo run -q -p ts2wasm-cli -- dump --ast --unparse fixtures/basics-types/satisfies-const-erasure.ts
cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/satisfies-const-erasure.ts -o /tmp/ts2wasm-059a.wasm
iwasm /tmp/ts2wasm-059a.wasm
mise run update-issue-index
mise run check issues
```

Not run:

- none

## Completion evidence

Commits:

- `1fbc95c` issue-059a: erase const assertions

Validation result:

```text
command: cargo test -p ts2wasm-frontend parses_typescript_const_assertions_as_erased_syntax -- --nocapture
result: pass; 1 passed
date: 2026-04-28

command: cargo test -p ts2wasm-cli --test dump_cli dump_ast_unparse_erases_typescript_satisfies_and_const_assertions -- --nocapture
result: pass; 1 passed
date: 2026-04-28

command: cargo test -p ts2wasm-cli --test dump_cli build_accepts_erasable_typescript_satisfies_and_const_assertions -- --nocapture
result: pass; 1 passed
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- dump --ast --unparse fixtures/basics-types/satisfies-const-erasure.ts
result: pass; output erases `satisfies`, `as const`, and `<const>`
date: 2026-04-28

command: cargo run -q -p ts2wasm-cli -- build fixtures/basics-types/satisfies-const-erasure.ts -o /tmp/ts2wasm-059a.wasm && iwasm /tmp/ts2wasm-059a.wasm
result: pass; stdout `3` and `7`
date: 2026-04-28

command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: cargo nextest run -p ts2wasm-frontend
result: pass; 59 passed
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli --test dump_cli
result: pass; 23 passed
date: 2026-04-28

command: mise run update-issue-index
result: pass; issues/index.md regenerated
date: 2026-04-28

command: mise run check issues
result: blocked; this worktree has no `check` task, supported issue gate is `check-issue-health`
date: 2026-04-28

command: mise run check-issue-health
result: failed due unrelated pre-existing missing gitignored report paths in issues/open/052-implement-json.md and issues/done/228-implement-logical-assignment-operators.md
date: 2026-04-28

command: scripts/manager check-agent-state
result: pass
date: 2026-04-28

command: cargo nextest run
result: pass; 408 passed, 4 skipped
date: 2026-04-28
```

Remaining risks:

- General angle-bracket type assertions remain outside this slice.
