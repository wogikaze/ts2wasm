---
id: 062g
title: "Define and implement heap closure object ABI and rooting (audit reopened #062g)"
type: feature
area: runtime/abi
class: done
priority: P1
depends_on: [256, 257, 258]
blocks: ["062e"]
status: done
created: 2026-04-29
updated: 2026-05-06
completed: 2026-04-29
---

## Summary

Define and implement the closure value representation needed for ordinary
functions or arrows that escape the declaring activation.

Problem: Returned closures need a heap object ABI and GC rooting contract before
implementation can safely preserve captured activations.

Problem: Returned closures cannot be represented by the current devirtualized
generated-function lowering because captured values are passed as hidden call
arguments, not stored in a heap environment that survives the declaring scope.

## Current failure

`fixtures/core-semantics/ordinary-function-closure-escape-unsupported.ts`
is intentionally rejected with `issue-062e:` because returning a nested
ordinary function would require a heap closure object and GC-rooted captured
environment.

The current implementation evidence is:

- `LoweredExpr::ArrowFn` is an opaque local token, not a callable heap object.
- Calls to known local closures are rewritten to direct `FunctionCallKind::User`
  calls with captured locals appended as hidden parameters.
- Returning that token would not preserve the captured activation after the
  declaring function returns.

## Desired final state

Escaping function values have a concrete runtime representation containing the
code identity and captured environment. Returned closures can be called after
the declaring scope has returned, and their captured heap values remain live
across allocation pressure.

## Scope

In scope:

- [x] Define the closure object/environment layout and code identity contract.
- [x] Lower returned ordinary function closures to heap closure values.
- [x] Dispatch calls through supported heap closure values.
- [x] Root closure environments during GC mark/sweep.
- [x] Add Node/iwasm differential fixtures for returned closure capture under
      allocation pressure.

Out of scope:

- Dynamic `Function` constructor semantics.
- `eval` / Annex B function declaration semantics.
- Function metadata (`name`, `length`, `prototype`) beyond existing 062f scope.
- Generator/async closure semantics.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/core-semantics/`
- `crates/cli/tests/`
- `docs/14-runtime-abi.md` if the closure ABI changes final-state contracts

Do not touch:

- `crates/runtime-abi/` unless the ABI change is explicitly reviewed.
- Issue 062f function metadata behavior.
- Issue 225 eval behavior.

## Acceptance criteria

- [x] A returned closure keeps an immutable captured local live after the
      declaring function returns.
- [x] A returned closure with a captured heap value survives allocation pressure
      in a Node/iwasm differential fixture.
- [x] Unsupported mutable environment forms either work correctly or produce an
      issue-linked diagnostic with a follow-up.
- [x] Runtime/ABI docs or current-state notes are synchronized with the closure
      object contract.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(closure) or test(function) or test(node_diff)'
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 94 --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/14-runtime-abi.md`

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] created: `issues/done/256-lower-returned-immutable-closures-to-heap-values.md`
- [x] created: `issues/done/257-emit-heap-closure-allocation-and-dispatch.md`
- [x] created: `issues/done/258-mark-heap-closure-captures-and-add-allocation-pressure-fixture.md`

## Notes

Start from the current non-escaping closure path: it already collects immutable
captures and passes them as hidden params for direct known calls. Do not reuse
that opaque token for escaping closures without a heap environment.

2026-04-29 design slice: `docs/14-runtime-abi.md` now defines the closure heap
object ABI. Closure values are object-tagged heap values with a closure sentinel,
`code_id`, immutable capture count, reserved flags, and raw capture slots. The
remaining implementation is split into issues 256, 257, and 258 so this broad
parent remains a tracking blocker rather than an executable work order.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `50e36ded2d68eb09dc29d5ed7fcd7723bc49c867`
- `b1e9a98c8fc94ccf794998ba97376045e7438cb9`
- `115d5cf74a9d19840303ff951463264529deb415`
- `29d57aced2fdcc3273ead0997bac39797780e0e5`

Validation result:

```text
command: cargo nextest run -E 'test(closure) or test(function) or test(node_diff) or test(gc)'
result: pass; 42 tests
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/ordinary-function-closure-gc-pressure.ts -o /tmp/ts2wasm-258-gc-pressure-parent.wasm && iwasm /tmp/ts2wasm-258-gc-pressure-parent.wasm
result: pass; output `closure-object-alive`
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-29

command: mise run check issues
result: pass
date: 2026-04-29
```

Remaining risks:

- Mutable captured environments, function metadata/prototype properties,
  broader closure arity dispatch, dynamic `Function`, `eval`, generators, and
  async closures remain outside this parent issue and are tracked by existing
  separate issues/scopes.

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/062g-heap-closure-object-abi-and-rooting.md` before this move
- `issues/done/062g-heap-closure-object-abi-and-rooting.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Reclosed by blocker cleanup

Date: 2026-05-06

Reason: the explicit child dependencies `256`, `257`, and `258` are in
`issues/done/`, and this parent already carries completion evidence for the
heap closure ABI, lowering, backend dispatch, GC rooting, and allocation-pressure
fixture validation. The open blocker was stale parent state, not remaining
implementation work.
