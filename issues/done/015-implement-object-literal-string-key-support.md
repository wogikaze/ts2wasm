# Implement object literal string key support (audit reopened #015)

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-26
**Completed**: 2026-04-26
**ID**: 015
**Type**: feature
**Area**: parser/semantics
**Priority**: P1
**Depends on**: none
**Orchestration class**: implementation-ready

Problem: Object literal with string literal keys `{"x": v}` is not implemented. Currently only identifier keys `{x: v}` are supported.

Scope:

- Extend parser to accept string literal keys in object literals.
- Lower string literal keys to runtime property keys.
- Add fixtures for string key object literals.
- Verify Node differential test passes.

Acceptance Criteria:

- [x] Parser accepts `{"key": value}` syntax.
- [x] String literal keys are correctly lowered to runtime properties.
- [x] Node differential test passes for string key fixtures.
- [x] No parse error for valid string key object literals.

Close:

- Date: 2026-04-26
- Evidence:
  - Added parse_object_key() method to accept both Token::Ident and Token::String as object keys
  - Updated object literal parsing to use parse_object_key() instead of expect_ident()
  - Test fixture fixtures/arrays-objects/string-key-literal.ts builds and runs correctly
  - iwasm output matches Node.js output ("Alice" and "30")
  - All tests pass: cargo nextest run (185 passed, 4 skipped)
  - Format check passes: cargo fmt --all --check

Validation:

```sh
cargo fmt --all --check
cargo nextest run
iwasm fixtures/arrays-objects/string-key-literal.wasm
```

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/015-implement-object-literal-string-key-support.md` before this close move
- `issues/done/015-implement-object-literal-string-key-support.md` after this close move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Child progress evidence

Date: 2026-05-05

- Parser implementation already uses `parse_object_key()` for object literal keys and accepts `Token::String`.
- Added parser regression coverage for `let obj = { "name": "Alice", "age": 30 };`.
- Added `fixtures/arrays-objects/string-key-literal.ts` to the M5 Node/iwasm differential fixture list.
- Verified Node-side fixture output with `node fixtures/arrays-objects/string-key-literal.ts`: `Alice` and `30`.
- Re-verification is blocked in this child shell because `cargo`, `iwasm`, `cargo-nextest`, and `wasm-tools` are not on PATH; `python scripts/manager.py fmt`, `python scripts/manager.py check`, targeted cargo tests, and `python scripts/manager.py check fixture-differential` cannot run to completion.
- Issue health checks passed: `python scripts/manager.py check issues` and `python scripts/manager.py update-issue-index --check`.

## Completion evidence

Date: 2026-05-05

- Parser regression `parses_string_literal_object_property_names` verifies `let obj = { "name": "Alice", "age": 30 };` parses into object properties `name` and `age`.
- `fixtures/arrays-objects/string-key-literal.ts` is included in `m5_array_object_fixtures_match_node_output_under_iwasm`, covering Node/iwasm differential execution for string literal object keys.
- Validation:
  - `PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py fmt` passed.
  - `PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py nextest parses_string_literal_object_property_names` passed: 1 test passed.
  - `PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py nextest m5_array_object_fixtures_match_node_output_under_iwasm` passed: 1 test passed.
  - `node fixtures/arrays-objects/string-key-literal.ts` printed `Alice` and `30`.
  - `PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH" python3 scripts/manager.py check issues` passed.
