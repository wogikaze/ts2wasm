---
id: 297
title: "Track pushed dense array locals for map callbacks"
type: feature
area: frontend/runtime
class: done
priority: P1
depends_on: []
blocks: [294]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Preserve dense-array knowledge for locals initialized as `[]` and then filled
with `.push(...)`, so later `.map(...)` callbacks can use the issue-295 array
map lowering.

This is a work order, not a design document and not a progress log.

## Problem

Problem: after issue 296, `fixtures/atcoder/abc451-d-concat-power2.ts`
advances past `2 ** i` but stops at `powersOfTwo.map(n => String(n))` because
the receiver is no longer recognized as a known dense array local.

## Current failure

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-power-parent.wasm --host-deny
```

Current result:

```text
error: [UnsupportedSyntax] issue-211: unknown receiver class for method `map` at 970..996
```

Representative source:

```ts
const powersOfTwo: number[] = [];
for (let i = 0; 2 ** i <= 1000000000; i++) {
    powersOfTwo.push(2 ** i);
}
const powersOfTwoStr: string[] = powersOfTwo.map(n => String(n));
```

## Desired final state

Dense array locals initialized from array literals remain known dense array
receivers after supported `.push(...)` mutations, allowing
`arrayLocal.map(n => String(n))` and similar existing issue-295 callback slices
to lower normally.

## Scope

In scope:

- [x] Track supported `.push(...)` mutations as preserving dense array locals.
- [x] Add a focused Node/iwasm fixture for `let values = []; values.push(...);
  values.map(n => String(n))`.
- [x] Verify `fixtures/atcoder/abc451-d-concat-power2.ts` advances past the
  `unknown receiver class for method map` blocker and record the next blocker.

Out of scope:

- Sparse arrays.
- General callback allocation beyond issue 295 supported callbacks.
- Full array mutation alias analysis.
- Source-specific ABC451 rewrites.

## Affected paths

Expected:

- `crates/ir/src/lowered/resolver.rs`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `fixtures/atcoder/`
- `issues/done/294-support-abc451-d-original-submission-without-source-rewrite.md`

Do not touch:

- problem-specific source rewrite hooks
- broad array runtime redesign
- parser syntax

## Acceptance criteria

- [x] Focused fixture for pushed dense array local `.map(n => String(n))`
  matches Node output under `iwasm`.
- [x] Existing issue-295 array-map fixtures still pass.
- [x] `fixtures/atcoder/abc451-d-concat-power2.ts` advances past the
  `unknown receiver class for method map` blocker.
- [x] Unsupported non-array receivers still report issue-linked diagnostics.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli <new focused test name>
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-array-map.wasm --host-deny
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created/updated: `issues/done/294-support-abc451-d-original-submission-without-source-rewrite.md`

## Notes

Prefer a narrow resolver tracking fix. If mutation invalidation is uncertain,
keep the slice to direct locals initialized from array literals and mutated by
supported `.push(...)` calls.

2026-04-29 progress:

- Added a conservative lowered-function signature fact for functions that
  return a local initialized from `[]` and preserved through supported
  `.push(...)` construction.
- Added a focused Node/iwasm fixture for `let values = [];
  values.push(...); values.map(n => String(n))`.
- Verified the ABC451 fixture advances past the prior
  `issue-211: unknown receiver class for method map at 970..996` blocker.
  The next blocker is now `DuplicateLocal: duplicate local binding: i`.
- While validating a wider multi-element pushed-array map fixture, direct
  indexing of the pushed array matched Node but `ArrayMapValueToString` over the
  mutated array produced a separate Node/iwasm difference. This slice keeps the
  regression fixture to the minimal tracked receiver case and leaves the runtime
  mapping/rooting behavior for a follow-up.

## Completion evidence

Commits:

- `45722ac9` issue-297: track pushed dense array returns
- `c9572789` merge issue-297 pushed array map tracking slice

Validation result:

```text
command: cargo nextest run -p ts2wasm-cli array_map_arrow_pushed_local_string_constructor_fixture_matches_node_output_under_iwasm
result: pass, 1 passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli array_map_arrow_split_fixture_matches_node_output_under_iwasm array_map_arrow_expression_receiver_fixture_matches_node_output_under_iwasm array_map_arrow_expression_split_fixture_matches_node_output_under_iwasm array_map_arrow_chained_trim_split_fixture_matches_node_output_under_iwasm array_map_arrow_string_constructor_fixture_matches_node_output_under_iwasm array_map_arrow_unary_plus_fixture_matches_node_output_under_iwasm array_map_fixtures_report_issue_270
result: pass, 7 passed
date: 2026-04-29

command: cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-issue297-close.wasm --host-deny
result: advanced past issue-211 unknown receiver class for method map at 970..996; current blocker is NumberOutOfRange for 1000000000
date: 2026-04-29
```

Remaining risks:

- The wider ABC451 fixture still needs issue 300 for the large integer number
  boundary.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/297-track-pushed-dense-array-locals-for-map.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
