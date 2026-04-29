---
id: 237
title: "Implement Annex B IsHTMLDDA compatibility"
type: feature
area: runtime/semantics
class: design-ready
priority: P3
depends_on: []
blocks: []
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
status: done
---

## Summary

Implement or explicitly diagnose ECMAScript Annex B `[[IsHTMLDDA]]` compatibility behavior used by test262 emulates-undefined cases.

## Problem

Issue 228 removed the `logical-assignment` parser/operator blocker from the limit-750 Annex B logical assignment files, but those reference cases still depend on test262/browser compatibility objects such as `$262.IsHTMLDDA` and `assert`. The current build stops earlier with `UnresolvedName` on `$262`, so full browser-compatible `document.all`-style semantics are not implemented.

## Desired final state

Annex B `[[IsHTMLDDA]]` values have a clear support policy. Supported cases match ECMAScript observable behavior for truthiness, nullish checks, and equality where in scope; unsupported host/browser compatibility forms produce precise issue-linked diagnostics.

## Scope

In scope:

- [x] Decide whether `[[IsHTMLDDA]]` is modeled directly, shimmed for reference tests, or rejected as unsupported browser compatibility.
- [x] Account for logical assignment cases under `reference/test262/test/annexB/language/expressions/logical-assignment/`.
- [x] Account for other known emulates-undefined cases such as `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js`.
- [x] Add regression fixtures or reference classification evidence for the selected policy.

Out of scope:

- [x] General test262 harness implementation beyond the minimum needed to classify these cases.
- [x] Logical assignment operator parsing and ordinary short-circuit semantics, completed by issue 228.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] The three Annex B logical-assignment emulates-undefined cases have a stable support or unsupported classification that no longer depends on incidental unresolved `$262` handling.
- [x] Unsupported `[[IsHTMLDDA]]` forms, if any remain, report a precise issue-linked diagnostic.
- [x] Regression fixtures or reference coverage evidence cover `&&=`, `||=`, `??=`, truthiness, and nullish behavior for the selected policy.
- [x] `cargo fmt --all --check` and `cargo nextest run` pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/expressions/logical-assignment/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated when behavior changes

Follow-up issues:

- [x] none

## Notes

Split from issue 228 close audit after the reference files stopped reporting the `logical-assignment` feature label and instead failed earlier on test262 harness name resolution.

2026-04-28 progress:

- Policy slice selected: unsupported host/browser compatibility form. The unshadowed test262 host hook `$262.IsHTMLDDA` now reports `UnsupportedSyntax` with `issue-237` instead of falling through to generic `UnresolvedName` on `$262`.
- Reference classification evidence: `TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/expressions/logical-assignment/ --detail` executes the three logical-assignment emulates-undefined cases and reports `unsupported=3`, `unsupported_diagcodes=UnsupportedSyntax:3`, `unsupported_features=logical-assignment:3`; per-file details cover `emulates-undefined-and.js`, `emulates-undefined-coalesce.js`, and `emulates-undefined-or.js`.
- Direct diagnostic evidence: building `reference/test262/test/annexB/language/expressions/logical-assignment/emulates-undefined-or.js` reports `[UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook`$262.IsHTMLDDA`is not modeled; document.all compatibility semantics are unsupported`.
- Regression fixture evidence: `fixtures/core-semantics/annexb-ishtmldda-unsupported.ts` plus `annexb_ishtmldda_host_hook_reports_issue_237` verifies the issue-linked diagnostic through the CLI.
- This is progress only. Full `[[IsHTMLDDA]]` truthiness/nullish/equality semantics, `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js`, and full `cargo nextest run` close evidence remain open.

2026-04-28 completion:

- Policy selected: reject Annex B `[[IsHTMLDDA]]` host/browser compatibility forms as unsupported rather than modeling or shimming `document.all` semantics.
- Commit `4a60707` adds focused unsupported fixtures for equality, `if` truthiness, `typeof`, and `&&=` / `||=` / `??=` RHS forms, all verified by `annexb_ishtmldda_host_hook_reports_issue_237`.
- Reference classification evidence: `TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/expressions/logical-assignment/ --detail` reports `unsupported=3`, `unsupported_diagcodes=UnsupportedSyntax:3`, and `unsupported_features=annexb-ishtmldda:3` for the `emulates-undefined-and.js`, `emulates-undefined-coalesce.js`, and `emulates-undefined-or.js` cases.
- Reference classification evidence: `TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/expressions/equals/emulates-undefined.js --path-filter annexB/language/expressions/typeof/emulates-undefined.js --path-filter annexB/language/statements/if/emulated-undefined.js --detail` reports `unsupported=3`, `unsupported_diagcodes=UnsupportedSyntax:3`, and `unsupported_features=annexb-ishtmldda:3`.
- Direct diagnostic evidence: building `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js` reports `[UnsupportedSyntax] issue-237: Annex B [[IsHTMLDDA]] test262 host hook`$262.IsHTMLDDA`is not modeled; document.all compatibility semantics are unsupported`.

## Completion evidence

Commits:

- `4a60707` issue-237: classify ishtmldda host hook cases

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-28

command: cargo nextest run -p ts2wasm-cli annexb_ishtmldda
result: pass; 1 passed, 234 skipped
date: 2026-04-28

command: TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/expressions/logical-assignment/ --detail
result: pass; unsupported=3; unsupported_diagcodes=UnsupportedSyntax:3; unsupported_features=annexb-ishtmldda:3
date: 2026-04-28

command: TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --path-filter annexB/language/expressions/equals/emulates-undefined.js --path-filter annexB/language/expressions/typeof/emulates-undefined.js --path-filter annexB/language/statements/if/emulated-undefined.js --detail
result: pass; unsupported=3; unsupported_diagcodes=UnsupportedSyntax:3; unsupported_features=annexb-ishtmldda:3
date: 2026-04-28

command: mise run check-scripts
result: pass
date: 2026-04-28

command: mise run check-agent-state
result: pass
date: 2026-04-28

command: cargo nextest run
result: pass; 380 passed, 4 skipped
date: 2026-04-28
```

Remaining risks:

- none
