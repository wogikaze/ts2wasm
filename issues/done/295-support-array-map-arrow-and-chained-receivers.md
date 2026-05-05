---
id: 295
title: "Support Array.map arrow callbacks and chained receivers"
type: feature
area: runtime/builtins
class: done
priority: P1
depends_on: []
blocks: [294]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement the Array.prototype.map subset needed by normal TypeScript code that
uses arrow callbacks and receivers produced by another expression, including the
ABC451 D input parsing path.

This is a work order, not a design document and not a progress log.

## Problem

Issue 270 is done for named function callback support, but its remaining risks
explicitly leave arrow callback support out of scope. ABC451 D now reaches that
gap after parser, empty export, and Bun stdin support.

Problem: `inputText.trim().split("\n").map(row => row.split(" "))` fails before
wasm generation because map currently requires an identifier receiver or reports
issue-270 for unsupported callback allocation semantics.

## Current failure

Reproduction:

```sh
cargo run -q -- build /tmp/abc451-original-bun.ts -o /tmp/abc451-original-bun.wasm --host-deny
```

Current diagnostic after commit `582b9d4f`:

```text
error: [UnsupportedSyntax] issue-211: method `map` requires an identifier receiver at 598..653
```

Minimal fixture shape:

```ts
const rows = inputText.trim().split("\n").map(row => row.split(" "));
const strings = values.map(n => String(n));
const numbers = strings.map(n => +n);
```

## Desired final state

Dense arrays support `.map(...)` when:

- the receiver is either an identifier local known to be an array or an
  expression that lowers to an array value such as `string.split(...)`;
- the callback is an arrow expression with one value parameter, or a named
  function callback supported by issue 270;
- the callback body can use the element value and return a string, number, or
  array value needed by the ABC451 D fixture.

The original ABC451 D source advances past the three `.map(...)` calls without
source-text rewriting.

## Scope

In scope:

- [x] Lower `.map(arrow)` over dense arrays to a wasm-side loop that allocates a
  new dense result array.
- [x] Accept receiver expressions that produce arrays, not only identifier
  receivers.
- [x] Cover `row => row.split(" ")`, `n => String(n)`, and `n => +n`.
- [x] Keep `Array.prototype.map.call(...)` unsupported unless the design is
  intentionally expanded.

Out of scope:

- Full sparse-array semantics.
- `thisArg`.
- Async callbacks or Promise handling.
- Source-specific replacement of the ABC451 program.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver.rs`
- `crates/ir/src/lowered/types.rs`
- `crates/backend-wasm/src/`
- `crates/cli/tests/m2_node_diff.rs`
- `fixtures/core-semantics/`
- `fixtures/atcoder/`
- `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`

Do not touch:

- problem-specific source rewrite hooks
- generated replacement implementations for a single contest task

## Acceptance criteria

- [x] A focused fixture with `["a b"].map(row => row.split(" "))` builds and
  matches Node output under `iwasm`.
- [x] Focused fixtures for `values.map(n => String(n))` and
  `values.map(n => +n)` build and match Node output under `iwasm`.
- [x] The original ABC451 D repro advances past all `.map(...)` calls.
- [x] Existing `Array.prototype.map.call(...)` unsupported diagnostics remain
  source-spanned.
- [x] No code path detects the ABC451 source text or substitutes another
  program.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python3 scripts/check/issue-health.py
```

Impacted commands:

```sh
cargo run -q -- build /tmp/abc451-original-bun.ts -o /tmp/abc451-original-bun.wasm --host-deny
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected
- not updated: `docs/05-compatibility-and-semantics.md`

Current state:

- [x] not affected
- not updated: `current-state.md` (repo root)

Follow-up issues:

- none: false; issue 294 was updated with the next blocker.
- [x] created/updated: `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`

## Notes

Prefer a general dense-array lowering path. If implementation needs a smaller
first slice, preserve issue-270's named-callback behavior and add arrow callback
support without widening to sparse arrays.

Progress on 2026-04-29:

- Added a focused inline dense array literal receiver slice for
  `["a b"].map(row => row.split(" "))`, covered by Node/iwasm differential
  fixture `fixtures/core-semantics/array-map-arrow-split.ts`.
- Added a focused expression receiver identity slice for
  `"a b".split(" ").map(part => part)`, covered by Node/iwasm differential
  fixture `fixtures/core-semantics/array-map-arrow-expression-receiver.ts`.
- Added a focused expression receiver split-map slice for
  `"a b\nc d".split("\n").map(row => row.split(" "))`, covered by
  `fixtures/core-semantics/array-map-arrow-expression-split.ts`.
- Added a focused `String(n)` callback slice for dense arrays such as
  `[1, -2, 0].map(n => String(n))`, covered by
  `fixtures/core-semantics/array-map-arrow-string-constructor.ts`.
- Deferred unary plus callback in this child because
  `crates/frontend/src/parser/expressions.rs` currently parses unary `+` as a
  no-op and the assignment's allowed files do not include frontend AST/parser
  changes needed to preserve numeric conversion semantics.
- Remaining issue-295 work: chained `trim().split(...).map(...)` receiver,
  plus unary plus callbacks.

Progress on 2026-04-29:

- Added a focused chained local string receiver fixture for
  `inputText.trim().split("\n").map(row => row.split(" "))`, covered by
  Node/iwasm differential test
  `array_map_arrow_chained_trim_split_fixture_matches_node_output_under_iwasm`.
- Verified `fixtures/atcoder/abc451-d-concat-power2.ts` now advances past the
  chained input parsing map and reaches the next blocker:
  `error: [UnsupportedSyntax] binary operator Power not yet supported`.
- Added a focused unary plus map callback slice for
  `strings.map(n => +n)`, covered by parser test
  `preserves_unary_plus_in_arrow_callback_body` and Node/iwasm differential
  fixture `fixtures/core-semantics/array-map-arrow-unary-plus.ts`.
- Parent verification confirmed the original ABC451 D repro advances past the
  issue-295 map callbacks and reaches the next blocker:
  `error: [UnsupportedSyntax] binary operator Power not yet supported`.

## Completion evidence

Commits:

- `50d34a68` / `36c99558`: inline dense array arrow split slice.
- `cd4d7a26`: identity map expression receiver slice.
- `8b8e71be`: split-map expression receiver slice.
- `0a445ae3` / `1e2594ac`: `String(n)` map callback slice.
- `3a199a6c` / `81dc26d2`: chained `trim().split(...).map(...)` fixture and ABC451 blocker update.
- `296588b` / `11c6705b`: unary plus map callback slice.

Validation result:

```text
command: cargo nextest run -p ts2wasm-cli array_map_arrow_unary_plus_fixture_matches_node_output_under_iwasm array_map_arrow_chained_trim_split_fixture_matches_node_output_under_iwasm array_map_arrow_string_constructor_fixture_matches_node_output_under_iwasm array_map_fixtures_report_issue_270
result: pass, 4 passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-frontend preserves_unary_plus_in_arrow_callback_body
result: pass, 1 passed
date: 2026-04-29

command: cargo fmt --all --check && mise run update-issue-index -- --check && mise run check issues
result: pass
date: 2026-04-29

command: cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-parent-295.wasm --host-deny
result: reached next blocker after map callbacks: error: [UnsupportedSyntax] binary operator Power not yet supported
date: 2026-04-29
```

Remaining risks:

- Full sparse-array semantics, `thisArg`, async callbacks, `Array.prototype.map.call(...)`,
  and full generic callback allocation remain out of scope for this issue.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/295-support-array-map-arrow-and-chained-receivers.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
