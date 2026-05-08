---
id: 067
title: "Investigate and classify unknown-unsupported cases (dup)"
type: spike
area: reference/triage
class: superseded
priority: P2
depends_on: []
blocks: []
created: 2026-04-26
updated: 2026-05-04
---

## Summary

Triage the generated reference bucket `Investigate and classify unknown-unsupported cases` before implementation. This issue records a failing reference case and must be split or superseded before any code change starts.

## Problem

Reference test results show 223 cases fail with unknown-unsupported diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: generated reference bucket `Investigate and classify unknown-unsupported cases` fails with `unknown-unsupported` and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js
```

Narrow coverage reproduction:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js --detail
```

Representative path: `reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js`
Feature label: `unknown-unsupported`

## Desired final state

This generated bucket is not used as a direct implementation work order. It is either superseded by an existing open/done issue, closed as a duplicate, or split into implementation-ready child issues that contain exact reproduction evidence and measurable acceptance criteria.

## Scope

In scope:

- [x] Run the representative `mise run reference-triage -- ...` command
- [x] Confirm whether duplicate candidates already cover this failure
- [x] Split one observable behavior or fixed reference window into child issues
- [x] Carry source context, diagnostic code, AST evidence, and validation commands into each child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad fixes that mix unrelated parser, resolver, runtime, and API failures

## Affected paths

Expected:

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

## Acceptance criteria

- [x] Duplicate candidates are confirmed as no-match, duplicate, or superseding issue
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

### Module resolution sub-classification

2026-05-03: Of the 223 unknown-unsupported cases, a significant subset fail due to UnsupportedModule (import/export syntax at top level). These are not parser bugs — they are blocked on module resolution (5007). Cases triaged as module in earlier sessions include accessorInferredReturnTypeErrorInReturnStatement (→ done/100) and aliasUsageInAccessorsOfClass (→ done/119). Remaining module-blocked cases await individual triage.

2026-04-28 child progress (`067-string-annexb-diagnostics-20260428T051924Z`):

- Commit: `3071f1cb44e51bbac0e264e8bee4de4d4bd7f1c6`
- Added a narrow issue-linked diagnostic for string-literal calls to Annex B `String.prototype.anchor`, `fontcolor`, `fontsize`, `link`, and `substr`.
- Added `fixtures/builtins-and-io/string-anchor-annexb-unsupported.ts` to cover `String.prototype.anchor`.
- Classified `/built-ins/String/` reference diagnostics and `String.prototype` diagnostic text as `string-builtin` instead of `unknown-unsupported` in CLI reference harnesses.
- Kept issue open; this is a diagnostic/classification slice only, not full Annex B implementation.

Validation:

```text
cargo fmt --all --check
result: pass

cargo test -p ts2wasm-cli --test m2_node_diff annex_b_string_anchor_fixture_reports_issue_067
result: pass

node fixtures/builtins-and-io/string-anchor-annexb-unsupported.ts
result: pass; stdout includes <a name="name">x</a>

cargo run -q -- build fixtures/builtins-and-io/string-anchor-annexb-unsupported.ts -o /tmp/ts2wasm-string-anchor-annexb-unsupported.wasm
result: expected fail; [UnsupportedSyntax] issue-067: Annex B String.prototype.anchor is not supported yet at 92..110

mise run check issues
result: pass

mise run check agent-state
result: pass

cargo nextest run
result: fail before completion in existing backend tests:
- ts2wasm-backend-wasm tests::function_locals_are_mirrored_into_activation_gc_root_frames
- ts2wasm-backend-wasm tests::top_level_locals_are_mirrored_into_gc_root_table
```

## Affected test files

- `reference/test262/test/annexB/built-ins/String/prototype/anchor/B.2.3.2.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontcolor/B.2.3.7.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/B.2.3.8.js`
- `reference/test262/test/annexB/built-ins/String/prototype/link/B.2.3.10.js`
- `reference/test262/test/annexB/built-ins/String/prototype/substr/surrogate-pairs.js`
- `reference/test262/test/annexB/built-ins/escape/escape-above-astral.js`
- `reference/test262/test/annexB/built-ins/escape/escape-above.js`
- `reference/test262/test/annexB/built-ins/escape/escape-below.js`
- `reference/test262/test/annexB/built-ins/unescape/four-ignore-bad-u.js`
- `reference/test262/test/annexB/built-ins/unescape/four.js`
- ... and 213 more files

## Duplicate detection

- `issues/open/201-implement-unknown-unsupported.md` - Investigate and classify unknown-unsupported cases (same feature label, title overlap)
- `issues/open/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, title overlap)


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/454-implement-unknown-unsupported.md` に統合されました。
そちらを参照してください。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/067-implement-unknown-unsupported.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
