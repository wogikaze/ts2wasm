---
id: 253
title: "Implement optional chaining runtime semantics (audit reopened #253)"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: ["246"]
blocks: []
created: 2026-04-29
updated: 2026-05-05
completed: 2026-04-29
status: open
---

## Summary

Implement lowering/runtime behavior for optional chaining forms that now parse as explicit AST nodes.

Problem: Issue 246 classifies `obj?.x`, `obj?.[key]`, and `fn?.()` in the frontend parser, but name resolution currently reports an issue-linked diagnostic because nullish short-circuit semantics are not lowered.

## Current failure

```sh
tmp=/tmp/ts2wasm-253-optional-runtime.ts
printf 'let obj = null;\nconsole.log(obj?.x);\n' > "$tmp"
cargo run -q -p ts2wasm-cli -- build "$tmp" -o /tmp/ts2wasm-253-optional-runtime.wasm
```

Expected current result:

```text
error: [UnsupportedSyntax] issue-246: optional chaining parses, but lowering/runtime semantics are not implemented
```

## Desired final state

Supported optional chaining forms short-circuit on `null` and `undefined` with Node-compatible observable behavior.

## Scope

In scope:

- [ ] Lower `obj?.x` for supported object/property access.
- [ ] Lower `obj?.[key]` for supported computed access.
- [ ] Lower `fn?.()` for supported function calls.
- [ ] Preserve single evaluation of the base expression for the implemented property/index subset.
- [ ] Add Node/iwasm differential fixtures for nullish and non-nullish bases for the implemented property/index subset.

Out of scope:

- Optional chaining on private names.
- `super?.x`, tagged templates, and `eval?.()` semantics unless split explicitly.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/`
- `docs/language-reference/javascript-features.md`

Do not touch:

- unrelated class runtime behavior
- parser-only issue files

## Acceptance criteria

- [ ] `obj?.x` returns `undefined` for nullish bases and the property value for supported objects.
- [ ] `obj?.[key]` preserves key evaluation only when required by the supported semantics.
- [ ] `fn?.()` short-circuits nullish callees and calls supported functions otherwise.
- [ ] Node/iwasm differential coverage proves the supported property/index subset.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/language/expressions/optional-chaining/ --detail
```

## Notes

Split from issue 246 so parser classification can close independently from runtime semantics.

## Progress evidence

2026-04-29:

- Implemented `obj?.x` and `obj?.[key]` lowering/runtime behavior for the supported object/index subset.
- Added `fixtures/core-semantics/optional-chaining-member-index.ts` and `optional_chaining_member_index_fixture_matches_node_output_under_iwasm`.
- Implemented `fn?.()` lowering/runtime behavior for the supported identifier-call subset: known-nullish locals short-circuit to `undefined`, and known local/function closures call normally when non-nullish.
- Added `fixtures/core-semantics/optional-chaining-call.ts` and `optional_chaining_call_fixture_matches_node_output_under_iwasm`.
- Validation passed for the focused optional-call differential test: `cargo nextest run -p ts2wasm-cli --test m2_node_diff optional_chaining_call_fixture_matches_node_output_under_iwasm` (1 passed).
- Validation passed: `cargo fmt --all --check`; `cargo nextest run -E 'test(optional) or test(node_diff)'` (6 passed); `cargo nextest run` (481 passed, 4 skipped); `mise run update-issue-index -- --check`; `mise run check issues`.
- Impacted reference coverage command could not run because `reference/test262` is not checked out in this worktree.

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/253-implement-optional-chaining-runtime-semantics.md` before this move
- `issues/open/253-implement-optional-chaining-runtime-semantics.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
