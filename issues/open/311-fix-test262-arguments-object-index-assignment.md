---
id: 311
title: "Fix test262 arguments object index assignment semantics"
type: bug
area: runtime/semantics
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

The previous test262 run has one P0 semantic failure in the executed window.
The failing case reaches wasm/iwasm execution but trips the test262 assertion for
an admitted `arguments` object index assignment.

This is a semantic correctness bug, not an unsupported-feature bucket.

## Problem

`arguments[7] = 12` should create or update the indexed property and a
subsequent `arguments[7]` read should observe `12`. The current wasm output
returns the test262 assertion sentinel instead.

Problem: arguments object out-of-range index assignment fails semantic comparison
in `reference/test262/test/language/arguments-object/10.5-7-b-2-s.js`.

## Current failure

Previous aggregate evidence:

```text
source: artifacts/coverage/results/test262.json
evidence: mise run reference-coverage -- test262 --limit 18000
executed=18000
fail=1
blocked=44
```

Focused reproduction:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference \
  python3 scripts/run/test262.py \
  --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/10.5-7-b-2-s.js \
  --jobs 1
```

Current result:

```text
Pass: 0
Fail: 1
Unsupported: 0
Blocked: 0
Total: 1
actual: "__TS2WASM_TEST262_ASSERT_FAIL__\n"
reason: Test262AssertionFailure: test262 assertion failed
```

Reference source excerpt:

```js
function _10_5_7_b_2_fun() {
    arguments[7] = 12;
    return arguments[7] === 12;
};

assert(_10_5_7_b_2_fun(30), '_10_5_7_b_2_fun(30) !== true');
```

## Desired final state

The representative test262 case passes under the test262 runner. Supported
`arguments` object indexed writes and reads match Node/iwasm differential
behavior for out-of-range indexes.

## Scope

In scope:

- [ ] Implement the smallest arguments-object indexed write/read semantics
      needed for `arguments[7] = 12; arguments[7]`.
- [ ] Preserve existing supported `arguments.length` and indexed read behavior.
- [ ] Add or update a focused fixture for out-of-range `arguments` index
      assignment.
- [ ] Make the representative test262 case pass or, if a narrower blocker is
      found, split that blocker with exact evidence and keep this issue open.

Out of scope:

- Full exotic arguments object aliasing semantics.
- Iterator support for arguments objects.
- Unrelated Object descriptor or callee/caller attributes.
- Broad test262 harness changes.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `artifacts/coverage/results/test262.json` only if the validation run is
  intentionally refreshed

Do not touch:

- `scripts/check/architecture-rules.py`
- `.githooks/pre-push`
- unrelated web-ui UI code

## Acceptance criteria

- [ ] Add an equivalent fixture for arguments-object out-of-range index writes
      and verify it matches Node output under iwasm.
- [ ] The representative test262 runner command reports `Pass: 1`, `Fail: 0`.
- [ ] `arguments.length` is not incorrectly extended unless the selected
      fixture proves the ECMAScript behavior requires it for this slice.
- [ ] Existing `function_arguments_fixture_matches_node_output_under_iwasm`
      coverage still passes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(arguments) or test(node_diff)'
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference python3 scripts/run/test262.py --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/10.5-7-b-2-s.js --jobs 1
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- test262 --limit 18000
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

This follows issue 288's remaining-risk note: the assert harness is now present
and the case reaches a real downstream arguments-object semantic failure.

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
