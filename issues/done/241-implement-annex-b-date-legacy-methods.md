---
id: 241
title: "Implement Annex B Date legacy methods"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P2
depends_on: []
blocks: ["050"]
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
status: done
---

## Summary

Implement or explicitly diagnose Annex B legacy Date methods tracked from the merged Date reference issue.

Problem: issue 061 was closed as a duplicate of the Date epic, but its Annex B Date reference evidence needs a closeable child issue instead of remaining only as a note on the broad epic.

## Current failure

Issue 050 preserves affected test262 evidence for:

- `Date.prototype.getYear`
- `Date.prototype.setYear`
- `Date.prototype.toGMTString`

The preserved cases include NaN return behavior, not-a-constructor checks, receiver validation, and setYear valid/invalid date-value handling.

## Desired final state

The listed Annex B Date methods either match ECMAScript Annex B behavior in supported scope or produce stable issue-linked unsupported diagnostics with reference coverage evidence.

## Scope

In scope:

- [x] Cover `Date.prototype.getYear` Annex B behavior or diagnostics.
- [x] Cover `Date.prototype.setYear` Annex B behavior or diagnostics.
- [x] Cover `Date.prototype.toGMTString` Annex B behavior or diagnostics.
- [x] Preserve deterministic Date epoch behavior for `getTime()` and `valueOf()`.

Out of scope:

- Live host time support for `new Date()` or `Date.now()`.
- Timezone-aware `Date.prototype.toString()` policy.
- Full Date API implementation beyond the listed Annex B methods.

## Affected paths

Expected:

- `crates/`
- `fixtures/`
- `issues/done/050-implement-date.md`
- `current-state.md`

Do not touch:

- unrelated parser syntax work

## Acceptance criteria

- [x] The affected Annex B Date test262 cases from issue 050/061 no longer rely on the broad Date epic as their only tracking target.
- [x] Supported behavior has Node/reference differential evidence, or unsupported behavior has issue-linked diagnostics.
- [x] Deterministic Date subset tests still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(date)'
mise run update-issue-index
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 500 --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- not affected unless Annex B policy is documented

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- none unless implementation reveals a separate Date object-model dependency

## Notes

The historical issue 061 diagnostic work used issue-061 wording. New diagnostics should point at this issue or another active child issue, not the closed duplicate.

Closed by retargeting the existing Annex B Date legacy method unsupported diagnostics
from closed duplicate issue 061 to active child issue 241. Full Annex B behavior
remains out of scope for the deterministic Date subset; the diagnostics are stable,
issue-linked, and covered by fixtures for `getYear`, `setYear`, and `toGMTString`.

## Completion evidence

Commits:

- `ce5259e` issue-241: close date annex b diagnostics
- parent merge commit: see repository history after integration

Validation result:

```text
command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-annexb-get-year-unsupported.ts -o /tmp/ts2wasm-date-annexb-get-year.wasm
result: failed as expected with `issue-241: Date.prototype.getYear is Annex B legacy Date behavior and is not supported in the deterministic Date epoch slice`
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-annexb-set-year-unsupported.ts -o /tmp/ts2wasm-date-annexb-set-year.wasm
result: failed as expected with `issue-241: Date.prototype.setYear is Annex B legacy Date behavior and is not supported in the deterministic Date epoch slice`
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/date-annexb-to-gmt-string-unsupported.ts -o /tmp/ts2wasm-date-annexb-to-gmt-string.wasm
result: failed as expected with `issue-241: Date.prototype.toGMTString is Annex B legacy Date behavior and is not supported in the deterministic Date epoch slice`
date: 2026-04-29

command: cargo nextest run -E 'test(date)'
result: passed; 16 tests passed, 404 skipped
date: 2026-04-29

command: cargo fmt --all --check
result: passed
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Date/prototype --detail
result: passed; executed=24, unsupported=24, unsupported_features=name-resolution:14,date:10
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --limit 500 --detail
result: passed; executed=500, unsupported=500, unsupported_features includes date:10
date: 2026-04-29

command: mise run update-issue-index
result: passed; issues/index.md updated
date: 2026-04-29

command: mise run update-issue-index -- --check
result: passed; issues/index.md OK
date: 2026-04-29

command: mise run check issues
result: pass in parent checkout; issues/index.md queue OK and check_issue_health OK
date: 2026-04-29

command: mise run check agent-state
result: passed
date: 2026-04-29

command: cargo nextest run
result: passed; 416 tests passed, 4 skipped
date: 2026-04-29
```

Remaining risks:

- The broad Date epic remains open for live time, timezone formatting, non-literal inputs, and full Date API behavior.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/241-implement-annex-b-date-legacy-methods.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
