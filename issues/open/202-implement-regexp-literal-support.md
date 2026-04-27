# Implement RegExp literal support

**Status**: open
**Created**: 2026-04-26
**Updated**: 2026-04-26
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

- [ ] RegExp literal syntax is accepted for at least basic literal + flags (`/abc/i`, `/a*/g`, escaped characters) in supported subset.
- [ ] Differential tests in `tests` pass for fixture coverage added under this issue.
- [ ] `unsupported_features.regexp-literal` contribution is reduced in `artifacts/coverage/results/test262.json` after next coverage run.
- [ ] New unsupported diagnostics, where still needed, include issue-linked reason code and fixture coverage evidence.

Validation:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py reference-coverage test262 --limit 50 --detail
```
