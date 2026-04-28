---
id: 228
title: "Implement logical assignment operators"
type: feature
area: frontend/semantics
class: design-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
---

## Summary

Implement JavaScript logical assignment operators (`&&=`, `||=`, and `??=`) with correct short-circuiting and assignment target behavior.

## Problem

The issue 060 test262 limit-750 classification window found 3 unsupported Annex B cases under `annexB/language/expressions/logical-assignment/`. These cases are now classified as `logical-assignment` instead of `unknown-unsupported`.

## Desired final state

Logical assignment expressions parse, lower, and execute according to ECMAScript semantics, including short-circuit evaluation and the special Annex B `[[IsHTMLDDA]]` emulates-undefined cases where supported.

## Scope

In scope:

- [ ] Parse logical assignment operators in assignment expressions.
- [ ] Preserve short-circuit evaluation and single evaluation of the assignment target.
- [ ] Lower and emit supported identifier/member logical assignment forms.
- [ ] Add regression fixtures for `&&=`, `||=`, and `??=`.

Out of scope:

- [ ] Broad assignment-target validation unrelated to logical assignment.
- [ ] Full host/browser `document.all` compatibility beyond precise unsupported diagnostics for `[[IsHTMLDDA]]` forms.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] The classified test262 logical-assignment cases no longer report `logical-assignment`.
- [ ] Regression fixtures cover `&&=`, `||=`, `??=`, skipped RHS evaluation, and single assignment-target evaluation.
- [ ] Unsupported `[[IsHTMLDDA]]` compatibility forms, if any remain, have precise issue-linked diagnostics.
- [ ] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --limit 750
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

Created from issue 060 classification evidence on 2026-04-28.

Reference-backed affected files in the limit-750 window:

- `reference/test262/test/annexB/language/expressions/logical-assignment/emulates-undefined-and.js`
- `reference/test262/test/annexB/language/expressions/logical-assignment/emulates-undefined-coalesce.js`
- `reference/test262/test/annexB/language/expressions/logical-assignment/emulates-undefined-or.js`

## Completion evidence

Fill only when moving to `done/`.

## Progress evidence

2026-04-28 child-worker slice:

- Implemented identifier-target `&&=`, `||=`, and `??=` parsing, lowering, and backend emission.
- Backend emission uses control-flow short-circuiting so skipped RHS calls are not evaluated.
- Added Node/iwasm differential fixture coverage for RHS skipped/evaluated behavior and final assigned values.
- Added an issue-linked diagnostic fixture for non-identifier logical assignment targets, which remain outside this slice.

Validation:

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(logical_assignment)'
result: pass; 2 tests passed

cargo nextest run -E 'test(parser)'
result: pass; 8 tests passed

cargo nextest run -E 'test(assignment)'
result: pass; 4 tests passed

node fixtures/core-semantics/logical-assignment.ts
result: pass; stdout matched iwasm output

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/logical-assignment.ts -o /tmp/ts2wasm-228-logical-assignment.wasm && iwasm /tmp/ts2wasm-228-logical-assignment.wasm
result: pass; stdout matched Node output

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass
```

Remaining risks:

- Member/index logical assignment targets are not implemented in this slice.
- The Annex B `[[IsHTMLDDA]]` test262 cases still require broader `$262`/HTMLDDA compatibility and are not closed by this progress slice.

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

2026-04-28 child-worker member continuation:

- Added static member logical assignment lowering/emission for identifier object targets, covering `obj.prop ||= rhs`.
- The member path reads the property once, short-circuits on truthy values, evaluates RHS only on the assignment branch, and writes back through the existing property runtime helper.
- Added Node/iwasm differential fixture coverage for skipped RHS and evaluated RHS behavior on `target.value ||= rhs(...)`.
- Kept issue-linked unsupported diagnostics for computed/index logical assignment targets.

Validation:

```text
cargo nextest run -E 'test(logical_assignment)'
result: pass; 3 tests passed

node fixtures/core-semantics/logical-assignment-member.ts
result: pass; stdout:
kept
kept
rhs
filled
filled

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/logical-assignment-member.ts -o /tmp/ts2wasm-228-logical-assignment-member.wasm
result: pass

iwasm /tmp/ts2wasm-228-logical-assignment-member.wasm
result: pass; stdout matched Node output

cargo fmt --all --check
result: pass

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass
```

Remaining risks:

- Computed/index logical assignment targets remain unsupported.
- Non-identifier member object targets such as `getObj().value ||= rhs()` remain unsupported to avoid claiming full single-evaluation semantics before dedicated temporary-target design.
- The Annex B `[[IsHTMLDDA]]` test262 cases still require broader `$262`/HTMLDDA compatibility and are not closed by this progress slice.

2026-04-28 child-worker computed string-literal continuation:

- Added computed string-literal logical assignment support for identifier receivers, covering forms such as `target["value"] ||= rhs(...)`.
- Reused the existing static property logical assignment lowering/emission path so object lookup still short-circuits and skipped branches do not evaluate RHS.
- Added Node/iwasm differential fixture coverage for `||=`, `??=`, and `&&=` through string-literal computed keys.
- Kept dynamic computed keys such as `target[key] &&= 1` unsupported with precise `issue-228` diagnostics because full dynamic-key single-evaluation needs a dedicated temporary-key design.

Validation:

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(logical_assignment)'
result: pass; 5 tests passed

node fixtures/core-semantics/logical-assignment-index.ts
result: pass; stdout:
kept
kept
rhs
filled
filled
rhs
fallback
fallback
rhs
again
again

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/logical-assignment-index.ts -o /tmp/ts2wasm-228-logical-assignment-index.wasm && iwasm /tmp/ts2wasm-228-logical-assignment-index.wasm
result: pass; stdout matched Node output

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass
```

Remaining risks:

- Dynamic computed logical assignment targets remain unsupported.
- Non-identifier receiver targets such as `getObj()["value"] ||= rhs()` remain unsupported to avoid claiming full single-evaluation semantics before dedicated temporary-target design.
- The Annex B `[[IsHTMLDDA]]` test262 cases still require broader `$262`/HTMLDDA compatibility and are not closed by this progress slice.
