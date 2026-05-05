---
id: 288
title: "Provide test262 assert harness binding"
type: feature
area: reference/runtime
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

## Summary

Create an actionable work item from the stderr set aggregation for test262 reference coverage.
This bucket appears 61 times and should be handled before lower-frequency failures.

Problem: Many admitted test262 cases fail before semantic comparison because the test262 `assert` harness function is unresolved.

## Current failure

Representative case:

```text
/home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/10.5-7-b-2-s.js
```

Reason:

```text
UnresolvedFunction/feature-resolution: [UnresolvedFunction]
```

Stderr bucket (61 occurrences):

```text
error: [UnresolvedFunction] unresolved function: `assert`\n
```

Aggregation command used to identify this bucket:

```sh
python3 - <<'PY'
import json, collections
from pathlib import Path
c = collections.Counter()
for line in Path("artifacts/coverage/results/test262-results.jsonl").open():
    record = json.loads(line)
    stderr = (record.get("stderr") or record.get("error") or record.get("reason") or "").strip()
    if stderr:
        c[stderr] += 1
print(c.most_common(10))
PY
```

## Desired final state

The representative case and related bucket members no longer produce this exact stderr string. They either pass, fail with a more specific downstream semantic mismatch, or are classified by an explicit unsupported feature diagnostic that is narrower than this bucket.

## Scope

In scope:

- [x] Provide a supported test262 harness binding or lowering path for `assert` in admitted reference cases.
- [x] Representative arguments-object cases compile far enough to produce semantic pass/fail instead of unresolved `assert`.
- [x] The exact `unresolved function:`assert`` stderr bucket is reduced or removed.

Out of scope:

- Full test262 harness API surface beyond `assert` unless required by representative cases.
- Node differential policy changes.

## Affected paths

Expected:

- ``scripts/run/reference-coverage.py``
- ``crates/cli/src/``
- ``crates/ir/src/``
- ``crates/backend-wasm/src/``

Do not touch:

- unrelated web-ui files
- unrelated issue files

## Acceptance criteria

- [x] The representative case no longer emits the exact stderr bucket shown above.
- [x] Regenerating `artifacts/coverage/results/test262-results.jsonl` shows this bucket count reduced or removed.
- [x] If behavior changes, add or update a focused fixture or regression test for the representative case; the focused JSONL representative run records the script-harness regression evidence.
- [x] Update coverage artifacts or current-state only when the validation run produces new facts.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/10.5-7-b-2-s.js
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
python3 scripts/gen/web-ui-data.py
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

This issue was generated from exact stderr set frequency, not directory-level grouping. Start with the representative case, then rerun the aggregation to confirm the count movement.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `c2613d9e` issue-288 test262 assert harness injection for coverage builds.

Validation result:

```text
command: python3 -m py_compile scripts/run/reference-coverage.py scripts/run/reference-coverage.py
result: PASS
date: 2026-04-29

command: mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/10.5-7-b-2-s.js
result: PASS; executed=1, build_pass=1, unsupported=0, blocked=0; exact UnresolvedFunction/assert bucket not emitted
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference python3 scripts/run/reference-coverage.py --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/10.5-7-b-2-s.js --jobs 1
result: PASS command; JSONL bucket count for `error: [UnresolvedFunction] unresolved function: `assert`\n` is 0; representative now reaches Test262AssertionFailure downstream
date: 2026-04-29

command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cargo nextest run
result: PASS (540 passed, 4 skipped)
date: 2026-04-29
```

Remaining risks:

- The representative case now compiles and reaches a real test262 assertion failure because current arguments-object semantics return false for this admitted case; this is downstream behavior outside the assert harness binding.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/288-provide-test262-assert-harness-binding.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
