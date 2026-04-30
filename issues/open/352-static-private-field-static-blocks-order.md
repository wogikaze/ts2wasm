---
id: 352
title: "Implement static private field ordering with static blocks"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: [255]
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Implement correct evaluation ordering for static private fields and static blocks in class definitions, ensuring they execute in source order alongside static public fields and static blocks.

## Problem

Static private fields (`static #x = 1`) and static blocks (`static { ... }`) must execute in class body source order. The current implementation supports individual static private fields and static blocks separately, but their relative ordering and interaction are not guaranteed. This is observable when a static block reads or writes a static private field.

## Current failure

```sh
tmp=/tmp/ts2wasm-352-static-order.ts
printf 'class C {\n  static #a = 1;\n  static { console.log(C.#a); }\n  static #b = 2;\n}\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-352-static-order.wasm
```

Current result: static blocks and static private fields may not execute in guaranteed source order; the above may produce incorrect output or fail to compile.

## Desired final state

Static private fields and static blocks execute in source order. A static block can read static private fields declared before it and set static private fields declared after it (per TDZ rules for the latter).

## Scope

In scope:

- [ ] Class-body static element ordering (public fields, private fields, blocks, methods)
- [ ] Static block lowering that can access static private fields
- [ ] TDZ enforcement for static private fields accessed before declaration in a static block
- [ ] Node/iwasm differential fixtures for static field/block ordering

Out of scope:

- Derived-class private elements (issue 350)
- Full brand storage (issue 351)
- Static accessor get/set duplicate-pair semantics

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/private-class*`

Do not touch:

- `crates/frontend/src/`

## Acceptance criteria

- [ ] Node/iwasm differential fixture proves static private field and block source ordering
- [ ] Node/iwasm differential fixture proves static block can read preceding static private field
- [ ] Diagnostic fixture proves TDZ violation for forward-referenced static private field in static block
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(private) or test(class) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo test -p ts2wasm-cli private
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] updated: `docs/language-reference/javascript-features.md` for static element ordering

Current state:

- [ ] updated: `current-state.md` if static element capability changes

Follow-up issues:

- [ ] none

## Notes

Parent issue: 255

ECMAScript class evaluation order for static elements: class heritage, class body (in source order: field initializers including private, then static blocks, then method definitions). Static private fields are initialized in order alongside static public fields and static blocks. A static block that references a static private field declared later in the class body should trigger a ReferenceError/TDZ.

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

- Static block lowering may need IR changes to represent class-body-level sequential execution
