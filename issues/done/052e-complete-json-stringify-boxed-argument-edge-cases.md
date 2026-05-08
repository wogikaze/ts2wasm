---
id: 052e
title: "Complete JSON.stringify boxed argument edge cases"
type: feature
area: runtime/builtins
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

Problem: `JSON.stringify` has validated narrow boxed `space` handling, but broader boxed and object-coercion edge cases are not closed.

## Summary

Audit and implement the remaining boxed/object argument cases for `JSON.stringify` `replacer` and `space` semantics, or split any large runtime-coercion dependency into narrower child issues.

## Current failure

Existing issue 052 evidence closes only narrow boxed `Number`, `String`, and `Boolean` `space` forms and leaves boxed forms beyond those fixtures as remaining gaps.

## Desired final state

Boxed values passed to `JSON.stringify` arguments either match Node for the supported runtime object model or produce precise unsupported diagnostics with a smaller tracking issue.

## Scope

In scope:

- [x] Audit boxed `Number`, `String`, `Boolean`, and object coercion paths for the `space` argument.
- [x] Audit boxed entries in array replacer property lists.
- [x] Add Node differential or diagnostic fixtures for each selected edge case.

Out of scope:

- Function replacer callback execution.
- General object model features not required to classify the selected boxed cases.
- Non-ASCII JSON string support.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/`
- `issues/done/052-implement-json.md`

Do not touch:

- `docs/`

## Acceptance criteria

- [x] Boxed `space` behavior beyond the currently covered literals is either Node-matching or explicitly diagnosed.
- [x] Boxed array replacer entries are either Node-matching or explicitly diagnosed.
- [x] Existing `json-stringify-space-boxed-symbol` coverage still passes.
- [x] Any deferred object-model dependency has a separate issue reference.

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
node fixtures/builtins-and-io/json-stringify-space-boxed-symbol.ts
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-space-boxed-symbol.ts -o /tmp/ts2wasm-json-space-boxed-symbol.wasm && iwasm /tmp/ts2wasm-json-space-boxed-symbol.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] update `current-state.md` if the supported JSON subset changes

Follow-up issues:

- [x] update `issues/done/052-implement-json.md`

## Completion evidence

Commits:

- `30a7d7d` (`issue-052e: classify json stringify boxed args`)

Validation result:

```text
command: node fixtures/builtins-and-io/json-stringify-space-boxed-symbol.ts
result: pass; Node output captured selected boxed/ignored space cases including new Number(-2), new Number(), new String(), new Boolean(false), and new Object()
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-space-boxed-symbol.ts -o /tmp/ts2wasm-json-space-boxed-symbol.052e.wasm && iwasm /tmp/ts2wasm-json-space-boxed-symbol.052e.wasm
result: pass; iwasm output matched Node for the boxed/ignored space fixture
date: 2026-04-29

command: node fixtures/builtins-and-io/json-stringify-replacer-array-boxed.ts
result: pass; Node printed boxed Number/String static array replacer output
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-boxed.ts -o /tmp/ts2wasm-json-replacer-array-boxed.052e.wasm && iwasm /tmp/ts2wasm-json-replacer-array-boxed.052e.wasm
result: pass; iwasm output matched Node for boxed Number/String static array replacer entries
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-boxed-unsupported.ts -o /tmp/ts2wasm-json-replacer-array-boxed-unsupported.052e.wasm
result: pass; build rejected boxed Boolean property-list entry with issue-052 UnsupportedSyntax diagnostic
date: 2026-04-29

command: cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-space-boxed-unsupported.ts -o /tmp/ts2wasm-json-space-boxed-unsupported.052e.wasm
result: pass; build rejected broader object-coercion space form with issue-052e UnsupportedSyntax diagnostic
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run -E 'test(json)'
result: pass; 18 tests run / 18 passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli json
result: pass; 15 tests run / 15 passed
date: 2026-04-29

command: cargo nextest run
result: pass; 424 tests run / 424 passed / 4 skipped
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

