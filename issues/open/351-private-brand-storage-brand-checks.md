---
id: 351
title: "Implement full private brand storage and brand-checking semantics"
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

Replace the current internal-slot private storage with full private brand storage per ECMAScript, and implement runtime brand-checking semantics so that accessing a private element on an object that does not have the correct brand throws a TypeError.

## Problem

The current implementation uses internal slots appended to class instances for private field storage. This lacks the ECMAScript brand concept: in Node, `class C { #x; } let o = {}; o.#x` throws TypeError because `o` lacks C's brand. The current implementation would either silently fail or access the wrong slot.

## Current failure

```sh
tmp=/tmp/ts2wasm-351-brand-check.ts
printf 'class C { #x = 1; }\nlet o = {};\ntry { console.log(o.#x); } catch (e) { console.log(e.name); }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-351-brand-check.wasm
```

Current result: external private access is rejected at compile time with an issue-255 diagnostic, so no runtime brand check exists.

## Desired final state

Every class with private elements has a unique brand. Runtime private element access checks the receiver's brand and throws TypeError on mismatch. Internal private slot layout is associated with the brand rather than appended unconditionally.

## Scope

In scope:

- [ ] Per-class private brand generation and storage
- [ ] Brand attachment to instances during construction
- [ ] Runtime brand-check helper for private field/method/accessor access
- [ ] TypeError throwing on brand mismatch for external private access attempts
- [ ] Node/iwasm differential fixtures for brand-check behavior

Out of scope:

- Derived-class private elements (issue 350)
- Static private field ordering with static blocks (issue 352)
- Syntax support changes (parser already handles `#x`)

## Affected paths

Expected:

- `crates/runtime-abi/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/private-class*`

Do not touch:

- `crates/frontend/src/`

## Acceptance criteria

- [ ] Node/iwasm differential fixture proves external private access throws TypeError
- [ ] Node/iwasm differential fixture proves same-class access succeeds
- [ ] Node/iwasm differential fixture proves subclass access fails (no inherited brand)
- [ ] Runtime helper tests cover brand check and TypeError throw paths
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

- [ ] updated: `docs/14-runtime-abi.md` for brand storage ABI
- [ ] updated: `docs/language-reference/javascript-features.md` for brand semantics

Current state:

- [ ] updated: `current-state.md` if runtime brand capability changes

Follow-up issues:

- [ ] none

## Notes

Parent issue: 255

In ECMAScript, each class definition with private elements creates a PrivateEnvironment containing private names. Each private name is associated with a brand (the class constructor). Instances created by that constructor carry the brand. Private field access (`o.#x`) checks that `o` has the brand of the class where `#x` was declared.

The current implementation rejects external private access at compile time. Moving to runtime brand checks requires lowering external private access to runtime brand-check calls, which may require changes to how private access is represented in IR.

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

- Runtime TypeError throwing requires JS exception object support, which may not be fully implemented
