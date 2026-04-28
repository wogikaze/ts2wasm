---
id: 052d
title: "Implement broader JSON.stringify replacer semantics"
type: feature
area: runtime/builtins
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

Problem: `JSON.stringify` currently supports a narrow object-literal array replacer subset and diagnoses function replacers and unsupported property-list contents.

## Summary

Implement replacer behavior beyond the validated string/numeric-literal object-literal subset, including function callbacks and broader property-list forms when the required call/property semantics are available.

## Current failure

Existing issue 052 evidence records issue-linked diagnostics for function replacer callbacks and unsupported array replacer contents/forms.

## Desired final state

`JSON.stringify(value, replacer)` follows ECMAScript replacer behavior for supported runtime values or emits precise issue-linked diagnostics only for explicitly unsupported forms.

## Scope

In scope:

- [ ] Implement or explicitly gate function replacer callbacks.
- [ ] Expand array replacer property lists beyond literal string/number entries.
- [ ] Preserve property-list ordering and duplicate suppression.
- [ ] Add Node differential or diagnostic coverage for every newly supported or intentionally unsupported form.

Out of scope:

- JSON number representation changes.
- UTF-16/surrogate string representation changes.
- General function-call semantics outside the replacer callback surface.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `crates/cli/tests/`
- `issues/open/052-implement-json.md`

Do not touch:

- `docs/`

## Acceptance criteria

- [ ] A function replacer fixture either matches Node or reports a precise issue-linked unsupported diagnostic with a narrower follow-up.
- [ ] Array replacer contents beyond string/numeric literals are implemented or diagnosed with precise coverage.
- [ ] Existing string/numeric-literal replacer fixtures still match Node.
- [ ] Unsupported diagnostics do not mask forms that this issue claims to support.

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
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-function-unsupported.ts -o /tmp/ts2wasm-json-replacer-function.wasm
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/json-stringify-replacer-array-unsupported.ts -o /tmp/ts2wasm-json-replacer-array-unsupported.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] update `current-state.md` if the supported JSON subset changes

Follow-up issues:

- [ ] update `issues/open/052-implement-json.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
