---
id: 249
title: "Implement class static block parser support"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: []
blocks: ["059"]
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
status: done
---

## Summary

Implement ECMA-262 class static block syntax in the frontend parser.

Problem: Class body parsing does not model `static { ... }` as `ClassStaticBlock`, so static initialization blocks are not represented as a distinct parser slice.

## Current failure

Representative reproduction:

```sh
tmp=/tmp/ts2wasm-249-static-block.ts
printf 'class C { static { console.log(1); } }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- dump --ast --unparse "$tmp"
```

Current result:

```text
error: [UnsupportedSyntax] expected identifier, got Some(SpannedToken { kind: LeftBrace, span: Span { start: 17, end: 18 } }) at 19..26
```

Spec refs:

- `reference/ecma262/spec.html`: ClassStaticBlock, ClassStaticBlockBody, ClassStaticBlockStatementList
- `reference/test262/test/language/expressions/class/elements/`
- `reference/test262/test/language/statements/class/`

## Desired final state

`static { ... }` in a class body parses as a class static block and has a stable semantic boundary: either supported lowering for the selected subset or an issue-linked unsupported diagnostic after parsing.

## Scope

In scope:

- [x] Parse `ClassStaticBlock` as a class body element.
- [x] Preserve statement-list spans inside the static block.
- [x] Reject invalid static-block-only forms according to parser-owned early checks where practical.
- [x] Add parser and CLI dump/diagnostic coverage for static blocks.

Out of scope:

- Private class elements; tracked by issue 248.
- Runtime ordering and execution semantics for static blocks unless selected as the supported subset.
- Top-level await or async class static block behavior.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- unrelated class runtime behavior unless required for stable diagnostics

## Acceptance criteria

- [x] `class C { static { console.log(1); } }` parses as a class static block or reports an explicit issue-linked unsupported semantic diagnostic after parsing.
- [x] `static {}` does not get misparsed as a static method or static field.
- [x] Static block statement spans are preserved in AST/dump diagnostics.
- [x] A focused test262 class-elements static-block reference slice no longer fails due to missing parser classification.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli --test dump_cli
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/class/elements/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated `docs/language-reference/javascript-features.md` for parser-supported/runtime-unsupported boundary

Current state:

- [x] not updated; runtime static block execution remains unsupported

Follow-up issues:

- [x] created `issues/done/254-implement-class-static-block-runtime-semantics.md`

## Notes

This is a frontend/parser wave child issue split from issue 059 and `docs/language-reference/frontend-parser-wave.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending parent commit

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-frontend
result: pass; 76 tests run, 76 passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli --test dump_cli
result: pass; 34 tests run, 34 passed
date: 2026-04-29

command: mise run reference-coverage -- test262 --path-filter reference/test262/test/language/statements/class/static-init-statement-list-optional.js --detail
result: pass; focused static-block reference slice is classified as UnsupportedSyntax/class after parser classification
date: 2026-04-29
```

Remaining risks:

- Runtime static block execution semantics were completed in issue 254.
