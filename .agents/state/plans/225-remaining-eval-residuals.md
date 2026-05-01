# Plan: Issue 225 Remaining Eval Residuals

## Current State

- Issue 349 (runtime shim): **Closed** ✓
- Issue 406 (existing binding residuals): **Closed** ✓
- `cargo fmt --all --check`: **Passes** ✓
- `cargo nextest run`: Passes (1 pre-existing failure in backend-wasm, unrelated)

## Remaining Problem

22 test262 Annex B eval cases still report `eval` as their feature label (classified as `UnsupportedSyntax: eval`).

The `eval` classification comes from `diagnostic.rs` `display_code()` which checks if the message contains "eval", "issue-302", or "issue-347". All 22 cases contain "eval" in their diagnostic message.

## Root Cause Analysis

The 22 cases fall into two categories:

### Category A: issue-406 existing-function patterns (~6 cases)

The `static_block_function_eval_expansion` in `crates/frontend/src/parser/statements_general.rs` fires the issue-406 error when:
- Prefix (before the first `{ function f() {} }` block) is non-empty
- Suffix (after the block) is non-empty AND contains non-block content (e.g., bare `function f() { ... }` declarations)
These cases need the eval expansion to handle bare function declarations in the suffix, or fall back to parsing the entire eval source as a fragment.

### Category B: skip-early-err + no-skip patterns (~16 cases)

The `find_static_eval_function_block` finds a `{` token followed by `function`, but the inner source is not a single function declaration (e.g., `let f = 123;{ function f() {} }`). The expansion returns `Ok(None)` and falls back to `parse_static_eval_fragment`, which fails because the eval source contains unsupported syntax (e.g., `assert.throws`, `let` inside block, etc.).

## Approach

**This cycle**: Create a follow-up issue documenting the 22 residual cases.
- Document classification evidence
- Do NOT implement code changes (too complex for a single cycle)
- Split the meta-issue 225 as the FSM recommends

## Risks

- The follow-up issue will be complex (involves parser expansion, resolver, and possibly backend)
- Not all 22 cases will be fixable by the same approach

## Acceptance

- [x] Research complete: 22 Annex B eval cases identified
- [ ] Follow-up issue created with precise case list
- [ ] Issue 225 updated to note split
- [ ] `cargo fmt` passes
- [ ] `cargo nextest` passes
