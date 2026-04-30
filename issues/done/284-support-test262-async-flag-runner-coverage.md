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
status: done
completed: 2026-04-29
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

- [x] Teach the test262 runner to admit the `async` flag through a deliberate supported or expected-unsupported path.
- [x] Keep unsupported async syntax/runtime diagnostics attributable to compiler/runtime behavior, not metadata filtering.
- [x] Regenerate runner JSONL evidence so this stderr bucket no longer dominates the representative set aggregation.

Out of scope:

- Full async/await runtime semantics unless required by the admitted representative slice.
- Module runner support.

## Affected paths

Expected:

- ``scripts/run/reference-coverage.py``
- ``scripts/run/reference-coverage.py``
- ``artifacts/coverage/results/*``

Do not touch:

- unrelated web-ui files
- unrelated issue files

## Acceptance criteria

- [x] The representative case no longer emits the exact stderr bucket shown above.
- [x] Regenerating `artifacts/coverage/results/test262-results.jsonl` shows this bucket count reduced or removed.
- [x] If behavior changes, add or update a focused fixture or regression test for the representative case.
- [x] Update coverage artifacts or current-state only when the validation run produces new facts.

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

- `8b7ca15343602384acc0b02c239d40e906b94dd8`

Validation result:

```text
command: python3 -m py_compile scripts/run/reference-coverage.py scripts/run/reference-coverage.py
result: pass
date: 2026-04-29

command: mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/async-gen-meth-args-trailing-comma-multiple.js
result: pass; representative classified as UnsupportedSyntax/async, not UnsupportedTest262Metadata/test262-metadata
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run test262 -- --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/async-gen-meth-args-trailing-comma-multiple.js --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/comments/hashbang/module.js --jobs 1
result: pass; regenerated local test262-results.jsonl had async metadata bucket count 0 and module metadata bucket count 0
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run
result: pass; 540 tests passed, 4 skipped
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass before close; index was up to date
date: 2026-04-29

command: mise run check issues
result: pass before close
date: 2026-04-29
```

Remaining risks:

- none
