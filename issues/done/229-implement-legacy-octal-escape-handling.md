---
id: 229
title: "Implement legacy octal escape handling"
type: feature
area: frontend
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

Implement or explicitly diagnose legacy octal escape sequences in Annex B template literal coverage.

## Problem

The issue 060 test262 limit-750 classification window found 2 unsupported cases under `annexB/language/expressions/template-literal/legacy-octal-escape-sequence-*`. These cases are now classified as `legacy-octal-escape` instead of `unknown-unsupported`.

## Desired final state

Legacy octal escape sequences in template literals are accepted or rejected according to ECMAScript strict and non-strict mode rules, with precise diagnostics for unsupported forms.

## Scope

In scope:

- [x] Decide the supported strict and non-strict behavior for legacy octal escapes in template literals.
- [x] Parse or explicitly diagnose legacy octal escape sequences in template literal text.
- [x] Add regression fixtures for strict and non-strict legacy octal escape behavior.
- [x] Preserve existing template literal interpolation behavior.

Out of scope:

- Full Unicode escape or UTF-16 parity work beyond the legacy octal cases.
- Broad template literal semantic changes not required by these cases.

## Affected paths

Expected:

- `crates/frontend/src/`
- `fixtures/`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] The classified test262 legacy-octal-escape cases no longer report `legacy-octal-escape`.
- [x] Strict and non-strict template literal legacy octal escape cases have regression coverage.
- [x] Existing template literal interpolation fixtures still pass.
- [x] `cargo fmt --all --check` and `cargo nextest run` pass.

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

- [x] not affected

Current state:

- [x] already reflected in `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

Created from issue 060 classification evidence on 2026-04-28.

Reference-backed affected files in the limit-750 window:

- `reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-non-strict.js`
- `reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js`

## Progress evidence

2026-04-28:

- Supported the selected non-strict slice: legacy octal escapes inside string literals parsed from template interpolation, e.g. `` `${'\07'}` `` cooks to `\u0007`.
- Preserved strict-mode behavior by emitting an issue-linked `UnsupportedSyntax` diagnostic for legacy octal string escapes in strict code.
- Added an issue-linked `UnsupportedSyntax` diagnostic for legacy octal escapes in template literal text such as `` `\07` ``.
- Added Node differential coverage for `fixtures/core-semantics/template-literal-legacy-octal.ts`.
- Added strict diagnostic regression coverage for `fixtures/core-semantics/template-literal-legacy-octal-strict-unsupported.ts`.
- Filtered reference evidence:

```text
command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter legacy-octal-escape-sequence --limit 750 --detail
result: PASS; unsupported_features no longer includes legacy-octal-escape. The two Annex B template legacy-octal cases now classify as name-resolution after escape handling.
date: 2026-04-28
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `6512527` issue-229: handle legacy octal template slice

Validation result:

```text
command: cargo fmt --all --check
result: passed
date: 2026-04-28

command: cargo test -p ts2wasm-frontend legacy_octal -- --nocapture
result: passed; 4 passed, including non-strict cooking, strict string rejection, strict interpolation rejection, and template-text rejection.
date: 2026-04-28

command: cargo test -p ts2wasm-cli --test m2_node_diff template_literal -- --nocapture
result: passed; 3 passed, including the existing interpolation fixture, non-strict legacy octal differential fixture, and strict issue-229 diagnostic fixture.
date: 2026-04-28

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference scripts/manager reference-coverage test262 --path-filter legacy-octal-escape-sequence --limit 750 --detail
result: passed; unsupported_features were name-resolution:4, template-literal:3, function:1, with no legacy-octal-escape label. The two Annex B template legacy-octal cases classified as name-resolution.
date: 2026-04-28

command: cargo nextest run
result: passed; 296 passed, 4 skipped
date: 2026-04-28
```

Remaining risks:

- none
