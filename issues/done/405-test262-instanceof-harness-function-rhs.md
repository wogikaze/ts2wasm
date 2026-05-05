---
id: 405
title: "Support Test262 harness instanceof function RHS"
type: feature
area: frontend/semantics
class: done
priority: P2
depends_on: [207]
blocks: [338]
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
status: done
---

## Summary

Support or explicitly shim the Test262 harness `instanceof` pattern that uses a
function-valued right-hand side rather than a lowered class constructor.

Problem: the sparse Array.map representative now advances past mutable callback
capture, but stops in the harness at an `instanceof` RHS diagnostic.

## Problem

Issue 207 completed ordinary prototype-chain `instanceof` for supported class
constructors. The Test262 assertion harness still contains function-constructor
RHS forms that are not represented as supported class constructors in the
current frontend/runtime subset.

## Current failure

```sh
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js
```

Current result after issue 404:

```text
[UnsupportedSyntax] issue-207: instanceof right-hand side must be a supported class constructor `Array`
```

## Desired final state

The selected Test262 sparse Array.map representative no longer stops at the
harness `instanceof` function-RHS boundary. It either compiles the required
ordinary-function constructor `instanceof` subset or the harness shim removes the
unsupported construct while preserving assertion semantics.

## Scope

In scope:

- [x] Clear the minimal issue-207 diagnostic reached by the selected
      representative. Local reproduction showed the prepared wasm source has no
      `instanceof` token; the issue-207 text came from the ordinary
      `new Array(10)` source shape falling through to class-constructor
      lowering.
- [x] Keep issue 207 ordinary class-constructor semantics intact.
- [x] Rerun the selected Array.map representative and update issue 338 with
      pass evidence.

Out of scope:

- Custom `Symbol.hasInstance`.
- Broad JavaScript function constructor/prototype semantics beyond the harness
  slice.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `scripts/lib/test262_harness.py`
- `issues/done/338-array-map-sparse-array-holes.md`

Do not touch:

- unrelated runtime builtin families

## Acceptance criteria

- [x] The selected representative no longer reports the issue-207 harness
      `instanceof` RHS diagnostic.
- [x] A focused regression or harness test covers the supported path.
- [x] Issue 338 is updated with the next concrete Test262 blocker or pass
      evidence.
- [x] `cargo fmt --all --check` and `mise run check issues` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo nextest run -p ts2wasm-cli array_map
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected unless broad `instanceof` semantics change

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Prefer a harness shim if the failing construct is only used to implement Test262
assertion helpers; prefer compiler/runtime support if an ordinary source fixture
needs the same behavior.

Implementation note: the selected prepared source uses ordinary
`var srcArr = new Array(10)` plus `srcArr[1] = undefined`. This slice therefore
adds narrow compiler support for small `new Array(length)` sparse arrays instead
of changing the Test262 harness shim or broadening issue 207 `instanceof`
semantics.

## Completion evidence

Fill only when moving to `done`.

Commits:

- `f65c30c3` `issue-338: add sparse new Array map progress`

Validation result:

```text
command: cargo test -p ts2wasm-cli array_map_new_array_holes_fixture_matches_node_output_under_iwasm -- --nocapture
result: pass; focused Node/iwasm fixture for `new Array(10)` sparse holes plus `Array.prototype.map` passed
date: 2026-05-01

command: cargo test -p ts2wasm-cli instanceof_unsupported_rhs_fixture_reports_issue_207 -- --nocapture
result: pass; issue 207 ordinary unsupported RHS diagnostic remains covered
date: 2026-05-01

command: cargo nextest run -p ts2wasm-cli array_map
result: pass; 18/18
date: 2026-05-01

command: mise run reference-triage -- test262 reference/test262/test/built-ins/Array/prototype/map/15.4.4.19-8-b-1.js
result: pass for issue-405 boundary; no longer reports `[UnsupportedSyntax] issue-207`, now reports `BuildPass`
date: 2026-05-01
```

Remaining risks:

- `reference-triage` is build-focused for this command. Issue 338 records the
  semantic/reference-coverage pass evidence for the sparse map representative.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/405-test262-instanceof-harness-function-rhs.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
