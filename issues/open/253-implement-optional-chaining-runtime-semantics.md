---
id: 253
title: "Implement optional chaining runtime semantics"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: ["246"]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement lowering/runtime behavior for optional chaining forms that now parse as explicit AST nodes.

Problem: Issue 246 classifies `obj?.x`, `obj?.[key]`, and `fn?.()` in the frontend parser, but name resolution currently reports an issue-linked diagnostic because nullish short-circuit semantics are not lowered.

## Current failure

```sh
tmp=/tmp/ts2wasm-253-optional-runtime.ts
printf 'let obj = null;\nconsole.log(obj?.x);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-253-optional-runtime.wasm
```

Expected current result:

```text
error: [UnsupportedSyntax] issue-246: optional chaining parses, but lowering/runtime semantics are not implemented
```

## Desired final state

Supported optional chaining forms short-circuit on `null` and `undefined` with Node-compatible observable behavior.

## Scope

In scope:

- [ ] Lower `obj?.x` for supported object/property access.
- [ ] Lower `obj?.[key]` for supported computed access.
- [ ] Lower `fn?.()` for supported function calls.
- [ ] Preserve single evaluation of the base expression.
- [ ] Add Node/iwasm differential fixtures for nullish and non-nullish bases.

Out of scope:

- Optional chaining on private names.
- `super?.x`, tagged templates, and `eval?.()` semantics unless split explicitly.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

Do not touch:

- unrelated class runtime behavior
- parser-only issue files

## Acceptance criteria

- [ ] `obj?.x` returns `undefined` for nullish bases and the property value for supported objects.
- [ ] `obj?.[key]` preserves key evaluation only when required by the supported semantics.
- [ ] `fn?.()` short-circuits nullish callees and calls supported functions otherwise.
- [ ] Node/iwasm differential coverage proves the supported subset.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/optional-chaining/ --detail
```

## Notes

Split from issue 246 so parser classification can close independently from runtime semantics.
