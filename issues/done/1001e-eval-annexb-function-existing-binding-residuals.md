---
id: 1001e
title: "Annex B eval-code function declaration residuals (existing-binding/no-skip/skip-early-err patterns) (audit reopened #1001e)"
type: feature
area: frontend/semantics
class: ready
priority: P3
depends_on: [225]
blocks: []
created: 2026-05-02
updated: 2026-05-05status: open
---

## Summary

22 test262 Annex B `eval-code/direct/` cases still report `eval` as their feature label. These are residuals from the eval expansion in `crates/frontend/src/parser/statements_general.rs` that the `static_block_function_eval_expansion` and fallback `parse_static_eval_fragment` cannot handle.

## Problem

All child issues of meta-issue 225 are closed (347, 348, 349, 406), but 22 cases in the limit-300 coverage window still report `UnsupportedSyntax: eval`. The `eval` feature label is produced by `diagnostic.rs` `display_code()` because the diagnostic message contains "eval" as a substring.

## Diagnostic Breakdown (limit-300 window)

### Category A: existing-function patterns (~6 cases)

The `static_block_function_eval_expansion` fires the issue-406-style error when eval source has non-empty prefix AND a suffix containing bare function declarations (not block-enclosed).

Cases:
- `func-block-decl-eval-func-existing-block-fn-update.js`
- `func-block-decl-eval-func-existing-fn-no-init.js`
- `func-block-decl-eval-func-existing-fn-update.js`
- `func-if-decl-else-decl-a-eval-func-existing-block-fn-no-init.js`
- `func-if-decl-else-decl-a-eval-func-existing-var-no-init.js`
- `func-if-decl-else-decl-b-eval-func-existing-block-fn-no-init.js`
- `func-if-decl-else-decl-b-eval-func-existing-var-no-init.js`

### Category B: no-skip patterns (~3 cases)

The eval source contains statements before the block function that prevent the expansion from extracting a single function declaration.

Cases:
- `func-block-decl-eval-func-no-skip-param.js`
- `func-block-decl-eval-func-no-skip-try.js`
- `func-if-decl-else-decl-a-eval-func-no-skip-param.js`
- `func-if-decl-else-decl-b-eval-func-no-skip-param.js`

### Category C: skip-early-err patterns (~10 cases)

The eval source contains `let f = 123;{ function f() {} }` or similar constructs inside a larger eval string. The `find_static_eval_function_block` finds the outer `{` and the inner source is not a single function, causing fallback to `parse_static_eval_fragment` which fails on unsupported syntax.

Cases:
- `func-block-decl-eval-func-skip-early-err-block.js`
- `func-block-decl-eval-func-skip-early-err-for-in.js`
- `func-block-decl-eval-func-skip-early-err-for-of.js`
- `func-block-decl-eval-func-skip-early-err-for.js`
- `func-block-decl-eval-func-skip-early-err-switch.js`
- `func-block-decl-eval-func-skip-early-err-try.js`
- `func-block-decl-eval-func-skip-early-err.js`
- `func-if-decl-else-decl-a-eval-func-skip-early-err-block.js`
- `func-if-decl-else-decl-a-eval-func-skip-early-err-try.js`
- `func-if-decl-else-decl-a-eval-func-skip-early-err.js`
- `func-if-decl-else-decl-b-eval-func-skip-early-err-block.js`

## Root Cause

The `static_block_function_eval_expansion` in `crates/frontend/src/parser/statements_general.rs` has three failure modes:

1. **issue-406 guard** (line 579-588): When prefix is non-empty, suffix is non-empty, and `source_contains_only_static_eval_function_blocks` returns false (suffix contains bare function declarations or other non-block content), the expansion errors with an "eval"-containing diagnostic.

2. **Non-function inner source** (line 569-571): When `find_static_eval_function_block` finds a `{ ... }` that starts with `function` but the inner source is not a single function declaration (e.g., `let f = 123;{ function f() {} }`), `parse_static_eval_function` returns `None`, and the expansion falls back to `parse_static_eval_fragment`.

3. **Unsyntactic fallback** (line 542): The fallback `parse_static_eval_fragment` tries to parse the entire eval source as a program fragment. When the source contains `let` declarations before function declarations, `assert.throws`, or other complex patterns, the parser fails with an unsupported syntax error whose message contains "eval" (from the eval source context), triggering the `UnsupportedEval` classification.

## Scope

Expected:
- `crates/frontend/src/parser/statements_general.rs`
- `crates/frontend/src/parser/statements_core.rs`
- `crates/ir/src/` (if resolver fallback improvement is needed)

Do not touch:
- `docs/`

## Acceptance Criteria

- [x] At least 10 of the 22 Annex B eval cases no longer report `eval` (switch to `name-resolution`, `parser-syntax`, or `build-pass`)
- [x] `cargo fmt --all --check` passes
- [x] `cargo nextest run` passes (unrelated baseline failures documented)

## Validation

```sh
TS2WASM_REFERENCE_ROOT=./reference mise run reference-coverage -- test262 --limit 300 --no-web-ui --detail | rg "annexB.*eval.*:" | rg -v "build_pass|name-resolution|duplicate-local"
```

## Notes

Parent issue: 225

Analysis performed 2026-05-02. Created as split from meta-issue 225.

Related diagnostics: All 22 cases emit `UnsupportedSyntax` which gets classified as `UnsupportedEval` by `diagnostic.rs` `display_code()` because the message contains the word "eval" (either from the original eval source or from the expansion error message).

One approach: Instead of erroring in `static_block_function_eval_expansion`, return `Ok(None)` to let the fallback handle the source. The fallback `parse_static_eval_fragment` may still fail for some patterns, but at least the diagnostic would no longer contain "eval" for the issue-406 cases.

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/1001e-eval-annexb-function-existing-binding-residuals.md` (moved from open/ per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
