---
id: 248
title: "Implement private class element parser support"
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

Implement parser recognition for ECMA-262 `PrivateIdentifier` and private class elements.

Problem: The lexer rejects `#` before the class parser can classify private fields or methods, so private class syntax reports an unsupported character diagnostic.

## Current failure

Representative reproduction:

```sh
tmp=/tmp/ts2wasm-248-private-class.ts
printf 'class C { #x = 1; }\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- dump --ast --unparse "$tmp"
```

Current result:

```text
error: [UnsupportedSyntax] unsupported character: # at 10..11
```

Spec refs:

- `reference/ecma262/spec.html`: PrivateIdentifier, ClassElementName, FieldDefinition
- `reference/test262/test/language/expressions/class/elements/`

## Desired final state

Private identifiers and private class elements are recognized by the lexer/parser. Unsupported private-field runtime behavior is reported after syntax classification with an issue-linked diagnostic rather than as an unknown character.

## Scope

In scope:

- [x] Tokenize `#name` as a private identifier with spans.
- [x] Parse private fields, private methods, private getters, and private setters in class bodies.
- [x] Parse static private fields/methods as syntax.
- [x] Reject invalid private identifier forms with stable diagnostics where parser-level checks own them.
- [x] Add parser and CLI diagnostic coverage for private class elements.

Out of scope:

- Implementing private field storage and runtime access semantics.
- Optional chaining of private fields.
- Decorators.
- Class static blocks; tracked separately by issue 249.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

Do not touch:

- unrelated class runtime behavior unless required for stable diagnostics

## Acceptance criteria

- [x] `class C { #x = 1; }` no longer reports `unsupported character: #`.
- [x] Private method/accessor syntax parses or reports an issue-linked unsupported diagnostic after private identifier tokenization.
- [x] Invalid private identifier forms are covered by diagnostics.
- [x] A focused reference slice under `reference/test262/test/language/expressions/class/elements/` no longer fails at the first `#` character.

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

- [x] not updated; runtime private class support remains unsupported

Follow-up issues:

- [x] created `issues/done/255-implement-private-class-element-runtime-semantics.md`

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

command: mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/class/elements/regular-definitions-rs-private-method-alt.js --detail
result: pass; focused private-method reference slice is classified as UnsupportedSyntax/class rather than the previous `#` tokenizer failure
date: 2026-04-29
```

Remaining risks:

- Runtime storage/access semantics are not implemented; tracked by issue 255.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/248-implement-private-class-element-parser.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
