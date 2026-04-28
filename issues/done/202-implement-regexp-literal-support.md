# Implement RegExp literal support

**Status**: done
**Created**: 2026-04-26
**Updated**: 2026-04-28
**Closed**: 2026-04-28
**ID**: 202
**Type**: feature
**Area**: frontend/semantics
**Priority**: P1
**Depends on**:
**Orchestration class**: implementation-ready

Problem: RegExp literals are currently reported as unsupported in the test262 coverage sweep (`unsupported_features.regexp-literal: 18`). This keeps semantic progress stuck and blocks adjacent language coverage slices in `language/expressions` and built-in regex syntax.

Scope:

- Add parser/codegen support for JavaScript RegExp literal syntax (`/pattern/flags`) in frontend lowering path.
- Route supported literal forms to existing runtime string/bytes handling and emit diagnostics for unsupported flag combinations.
- Add fixtures under `fixtures/` for basic and boundary RegExp literal cases used by differential verification.
- Update coverage-oriented issue tracking and add unsupported-diag tags as needed.

Acceptance Criteria:

- [x] RegExp literal syntax is accepted for at least basic literal + flags (`/abc/i`, `/a*/g`, escaped characters) in supported subset.
- [x] Differential tests in `tests` pass for fixture coverage added under this issue.
- [x] `unsupported_features.regexp-literal` contribution is reduced in `artifacts/coverage/results/test262.json` after next coverage run.
- [x] New unsupported diagnostics, where still needed, include issue-linked reason code and fixture coverage evidence.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --limit 50 --detail
```

## Progress evidence

- 2026-04-28: Existing frontend implementation tokenizes RegExp literals as `Token::RegExp` and parser support routes supported literals to the existing string-backed expression subset.
- 2026-04-28: `fixtures/core-semantics/regexp-literal.ts` covers `/abc/i`, `/a*/g`, escaped slash, and slash inside a character class with Node/iwasm differential coverage.
- 2026-04-28: `fixtures/core-semantics/regexp-unsupported-flag.ts` verifies unsupported flag diagnostics remain issue-linked.
- 2026-04-28: `cargo fmt --all --check` passed.
- 2026-04-28: `cargo nextest run -E 'test(regexp)'` passed: 6 tests run, 6 passed.
- 2026-04-28: `mise run reference-coverage -- test262 --limit 50 --detail` passed after initializing ignored `reference/test262`; measured `unsupported_features.regexp-literal:13`, reduced from the checked-in artifact baseline of 18.
- 2026-04-28: `artifacts/coverage/results/test262.json` updated with the measured limit-50 coverage result.

## Completion evidence

- 2026-04-28: Parent-authorized close updated issue 009's historical reference from the open path to this done path.
- 2026-04-28: `mise run update-issue-index -- --check` passed after moving this issue to done.
- 2026-04-28: `mise run check-issue-health` passed after moving this issue to done.
