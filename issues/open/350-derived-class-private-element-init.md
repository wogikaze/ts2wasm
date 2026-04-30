---
id: 350
title: "Implement derived-class private element initialization"
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

Implement runtime initialization and storage for private class elements (fields, methods, accessors) in derived classes, including correct super-constructor ordering and brand initialization.

## Problem

Private elements in derived classes must be initialized after the super constructor completes but before the derived class constructor body runs. The current implementation only supports non-derived classes. Attempting to use private elements in a derived class fails with an issue-255 diagnostic.

Problem: Derived-class private elements are rejected with an unsupported diagnostic.

## Current failure

```sh
tmp=/tmp/ts2wasm-350-derived-private.ts
printf 'class Base { constructor() { this.x = 1; } }\nclass Derived extends Base { #value = 2; }\nconsole.log(new Derived());\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-350-derived-private.wasm
```

Current result: `[UnsupportedSyntax] issue-255: private fields are not supported in derived classes in this private field runtime slice`

## Desired final state

Derived classes with private fields, methods, getters, and setters compile and execute with Node-compatible semantics. Private element initialization happens in the correct constructor phase.

## Scope

In scope:

- [ ] Derived-class constructor IR lowering with private slot allocation after super()
- [ ] Private field initialization ordering relative to public fields and super constructor
- [ ] Private method/accessor brand binding for derived-class instances
- [ ] Node/iwasm differential fixtures for derived-class private elements

Out of scope:

- Full private brand storage overhaul (issue 351)
- Static private field ordering with static blocks (issue 352)
- External/extracted private access (remains diagnostic-only)

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/private-class*`

Do not touch:

- `crates/frontend/src/`
- `crates/frontend/src/parser/`
- Parser private syntax tokenization

## Acceptance criteria

- [ ] Node/iwasm differential fixture for derived class with private field
- [ ] Node/iwasm differential fixture for derived class with private method
- [ ] Node/iwasm differential fixture for derived class with private getter/setter
- [ ] Regression fixture proving private initialization order (after super, before constructor body)
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

- [ ] updated: `docs/language-reference/javascript-features.md` if derived private element semantics are documented

Current state:

- [ ] updated: `current-state.md` if runtime private element capability changes

Follow-up issues:

- [ ] none

## Notes

Parent issue: 255

In ECMAScript, private fields are initialized in the constructor after `super()` returns but before any explicit constructor body code. If the derived class has no explicit constructor, private fields still initialize after the implicit `super()` call. Private methods and accessors share the same brand as private fields but do not need per-instance initialization beyond brand binding.

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

- Constructor execution order may interact with existing super() lowering in non-trivial ways
