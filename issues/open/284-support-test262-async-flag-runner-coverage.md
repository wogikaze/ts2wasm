---
id: 284
title: "Support test262 async flag in reference coverage"
type: test
area: reference/tests
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Create an actionable work item from the stderr set aggregation for test262 reference coverage.
This bucket appears 2844 times and should be handled before lower-frequency failures.

Problem: The largest stderr bucket is the reference runner rejecting every test262 case with the `async` flag before compiler/runtime behavior can be measured.

## Current failure

Representative case:

```text
/home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/async-gen-meth-args-trailing-comma-multiple.js
```

Reason:

```text
UnsupportedTest262Metadata/test262-metadata: test262 flag `async` is not supported by this runner slice
```

Stderr bucket (2844 occurrences):

```text
UnsupportedTest262Metadata/test262-metadata: test262 flag `async` is not supported by this runner slice
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

- [ ] Teach the test262 runner to admit the `async` flag through a deliberate supported or expected-unsupported path.
- [ ] Keep unsupported async syntax/runtime diagnostics attributable to compiler/runtime behavior, not metadata filtering.
- [ ] Regenerate web-ui data so this stderr bucket no longer dominates the set aggregation.

Out of scope:

- Full async/await runtime semantics unless required by the admitted representative slice.
- Module runner support.

## Affected paths

Expected:

- ``scripts/run/test262.py``
- ``scripts/run/reference-coverage.py``
- ``artifacts/coverage/results/*``

Do not touch:

- unrelated web-ui files
- unrelated issue files

## Acceptance criteria

- [ ] The representative case no longer emits the exact stderr bucket shown above.
- [ ] Regenerating `artifacts/coverage/results/test262-results.jsonl` shows this bucket count reduced or removed.
- [ ] If behavior changes, add or update a focused fixture or regression test for the representative case.
- [ ] Update coverage artifacts or current-state only when the validation run produces new facts.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/async-gen-meth-args-trailing-comma-multiple.js
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

- [ ] not affected
- [ ] updated: `docs/...`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none
- [ ] created/updated: `issues/open/...`

## Notes

This issue was generated from exact stderr set frequency, not directory-level grouping. Start with the representative case, then rerun the aggregation to confirm the count movement.

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
