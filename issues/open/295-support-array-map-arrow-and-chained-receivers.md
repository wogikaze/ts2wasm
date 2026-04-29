---
id: 295
title: "Support Array.map arrow callbacks and chained receivers"
type: feature
area: runtime/builtins
class: implementation-ready
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

- [ ] Lower `.map(arrow)` over dense arrays to a wasm-side loop that allocates a
  new dense result array.
- [ ] Accept receiver expressions that produce arrays, not only identifier
  receivers.
- [ ] Cover `row => row.split(" ")`, `n => String(n)`, and `n => +n`.
- [ ] Keep `Array.prototype.map.call(...)` unsupported unless the design is
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

- [ ] A focused fixture with `["a b"].map(row => row.split(" "))` builds and
  matches Node output under `iwasm`.
- [ ] Focused fixtures for `values.map(n => String(n))` and
  `values.map(n => +n)` build and match Node output under `iwasm`.
- [ ] The original ABC451 D repro advances past all `.map(...)` calls.
- [ ] Existing `Array.prototype.map.call(...)` unsupported diagnostics remain
  source-spanned.
- [ ] No code path detects the ABC451 source text or substitutes another
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

- [ ] not affected
- [ ] updated: `docs/05-compatibility-and-semantics.md`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none
- [ ] created/updated: `issues/open/294-support-abc451-d-original-submission-without-source-rewrite.md`

## Notes

Prefer a general dense-array lowering path. If implementation needs a smaller
first slice, preserve issue-270's named-callback behavior and add arrow callback
support without widening to sparse arrays.

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
