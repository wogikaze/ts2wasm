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
completed: 2026-04-29
status: done
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

- [x] Add expression parser support for `CoalesceExpression`.
- [x] Preserve the syntax restriction that unparenthesized `??` cannot be directly mixed with `&&` or `||`.
- [x] Lower the supported subset to null/undefined checks with short-circuit evaluation, or emit a stable issue-linked diagnostic for unsupported operand forms.
- [x] Add parser and differential coverage for `null ?? x`, `undefined ?? x`, and falsy non-nullish values.

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

- [x] `console.log(null ?? 2);` and `console.log(undefined ?? 2);` parse and produce Node-matching stdout in a focused differential fixture.
- [x] `console.log(false ?? 2);`, `console.log(0 ?? 2);`, and `console.log("" ?? 2);` preserve the left operand in differential coverage.
- [x] `a ?? b || c` and `a || b ?? c` are rejected unless parenthesized.
- [x] `reference/test262/test/language/expressions/coalesce/follows-null.js` no longer fails due to the current parser comma error.

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

- [x] updated `docs/language-reference/javascript-features.md` for the supported `??` subset

Current state:

- [x] not updated in this child slice because the assignment allowed paths did not include `current-state.md`

Follow-up issues:

- [x] none; no unsupported operand forms remain for the scoped primitive/local expression subset

## Notes

This is a frontend/parser wave child issue split from issue 059 and `docs/language-reference/frontend-parser-wave.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `ec0d352` issue-245: implement nullish coalescing frontend support

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-frontend
result: pass; 62 tests run, 62 passed
date: 2026-04-29

command: node fixtures/core-semantics/nullish-coalescing.ts
result: pass; stdout matched iwasm for nullish/falsy regression fixture
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/nullish-coalescing.ts -o /tmp/ts2wasm-245-nullish.wasm && iwasm /tmp/ts2wasm-245-nullish.wasm
result: pass; stdout matched Node for `null ?? 2`, `undefined ?? 2`, falsy left operands, and RHS evaluation count
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli m3_semantic_fixtures_match_node_output_under_iwasm --test-threads=1
result: pass; 1 test run, 1 passed
date: 2026-04-29

command: cargo nextest run
result: pass; 429 tests run, 429 passed, 4 skipped
date: 2026-04-29

command: mise run update-issue-index
result: pass; Updated issues/index.md
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK
date: 2026-04-29

command: mise run check issues
result: pass; issues/index.md queue OK and check_issue_health OK after recreating gitignored local report placeholders referenced by prior issues
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/coalesce/follows-null.js --detail
result: pass; follows-null.js is now classified as UnresolvedName/name-resolution instead of the previous parser comma error
date: 2026-04-29
```

Remaining risks:

- Broader test262 coalesce cases still depend on reference harness name-resolution support; this issue's parser comma failure is resolved.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/245-implement-nullish-coalescing-frontend.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
