---
id: 245
title: "Implement nullish coalescing frontend support"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: ["059"]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement ECMA-262 `??` expression parsing and the narrow frontend/lowering behavior needed for nullish coalescing.

Problem: The lexer recognizes `??`, but the parser does not accept it in expression grammar, so `a ?? b` reports `parser-syntax`.

## Current failure

Representative reproduction:

```sh
tmp=/tmp/ts2wasm-245-coalesce.ts
printf 'let a = null; let b = 2;\nconsole.log(a ?? b);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- dump --ast --unparse "$tmp"
```

Current result:

```text
error: [UnsupportedSyntax] expected Comma, got Some(NullishCoalesce) at 39..41
```

Spec refs:

- `reference/ecma262/spec.html`: CoalesceExpression, CoalesceExpressionHead
- `reference/test262/test/language/expressions/coalesce/`

## Desired final state

`??` parses with ECMA-262 precedence and short-circuit behavior. Parser-only success does not count as semantic parity unless Node differential tests prove the supported subset.

## Scope

In scope:

- [ ] Add expression parser support for `CoalesceExpression`.
- [ ] Preserve the syntax restriction that unparenthesized `??` cannot be directly mixed with `&&` or `||`.
- [ ] Lower the supported subset to null/undefined checks with short-circuit evaluation, or emit a stable issue-linked diagnostic for unsupported operand forms.
- [ ] Add parser and differential coverage for `null ?? x`, `undefined ?? x`, and falsy non-nullish values.

Out of scope:

- Optional chaining; tracked by issue 246.
- Annex B `[[IsHTMLDDA]]` behavior beyond the existing policy.
- Broad control-flow or optimizer rewrites.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

Do not touch:

- unrelated builtins
- capability manifest policy

## Acceptance criteria

- [ ] `console.log(null ?? 2);` and `console.log(undefined ?? 2);` parse and produce Node-matching stdout in a focused differential fixture.
- [ ] `console.log(false ?? 2);`, `console.log(0 ?? 2);`, and `console.log("" ?? 2);` preserve the left operand in differential coverage.
- [ ] `a ?? b || c` and `a || b ?? c` are rejected unless parenthesized.
- [ ] `reference/test262/test/language/expressions/coalesce/follows-null.js` no longer fails due to the current parser comma error.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/coalesce/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] update `docs/language-reference/javascript-features.md` when support status changes

Current state:

- [ ] update `current-state.md` if semantic support changes

Follow-up issues:

- [ ] none unless unsupported operand forms remain

## Notes

This is a frontend/parser wave child issue split from issue 059 and `docs/language-reference/frontend-parser-wave.md`.

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
