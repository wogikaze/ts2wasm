---
id: 052c
title: "Implement JSON UTF-16 and surrogate string handling"
type: feature
area: runtime/builtins
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

Problem: JSON string parsing currently supports ASCII and ASCII-valued `\uXXXX` escapes, but rejects non-ASCII code points and surrogate forms that Node accepts.

## Summary

Extend JSON string parse/stringify behavior beyond the current byte-oriented ASCII subset so UTF-16 escape decoding, non-ASCII strings, and surrogate-pair handling match the selected JavaScript string representation.

## Current failure

Existing issue 052 evidence records Node accepting unsupported unicode and surrogate fixtures while iwasm rejects them with `Exception: unreachable`.

## Desired final state

`JSON.parse` and `JSON.stringify` preserve JSON string values across non-ASCII code points and surrogate-pair cases according to the project's string representation contract.

## Scope

In scope:

- [x] Decode `\uXXXX` escapes for non-ASCII code points.
- [x] Handle surrogate pairs consistently with Node-observable behavior.
- [x] Preserve or explicitly define behavior for lone surrogates.
- [x] Add Node differential coverage for top-level, array-value, and object-value string paths.

Out of scope:

- General-purpose UTF-16 refactors unrelated to JSON.
- Number representation work.
- Replacer callback semantics.

## Affected paths

Expected:

- `crates/backend-wasm/src/`
- `crates/ir/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/`
- `issues/open/052-implement-json.md`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] `JSON.parse('"\\u00e9"')` matches Node for observable output.
- [x] `JSON.parse('"\\ud83d\\ude00"')` matches Node for observable output.
- [x] Object and array string-value paths match the top-level behavior.
- [x] Existing ASCII escape and invalid unicode escape fixtures still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(json)'
cargo nextest run -p ts2wasm-cli json
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
node fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-parse-unsupported-unicode-array.ts -o /tmp/ts2wasm-json-unicode-array.wasm && iwasm /tmp/ts2wasm-json-unicode-array.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] update `current-state.md` if the supported JSON subset changes

Follow-up issues:

- [x] update `issues/open/052-implement-json.md`

## Completion evidence

Commits:

- `ec9ffa4`

Validation result:

```text
command: cargo nextest run -E 'test(json)'
result: pass, 18 tests run / 18 passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli json
result: pass, 15 tests run / 15 passed
date: 2026-04-29

command: direct Node/iwasm checks for json-parse-unsupported-unicode-array.ts, json-parse-unsupported-unicode-object.ts, json-parse-unsupported-surrogate-pair.ts, json-parse-unsupported-surrogate-low.ts, and json-parse-surrogate-pair-object-array.ts
result: pass, Node and iwasm stdout matched; invalid unicode escape still rejected by both Node and iwasm
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run
result: pass, 416 tests run / 416 passed / 4 skipped
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass
date: 2026-04-29

command: mise run check agent-state
result: pass
date: 2026-04-29

command: mise run check issue-index; mise run check issues
result: fail due pre-existing missing reports/runs references in issues/open/052-implement-json.md historical notes and unrelated issues/done/228-implement-logical-assignment-operators.md; 052c done-file checklist/index state is clean.
date: 2026-04-29
```

Remaining risks:

- Lone surrogate escapes cannot be preserved as UTF-16 code units in the current byte-backed string representation; they materialize as U+FFFD for current observable stdout/string behavior. Full UTF-16 string storage remains outside this child issue.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/052c-implement-json-utf16-surrogate-strings.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
