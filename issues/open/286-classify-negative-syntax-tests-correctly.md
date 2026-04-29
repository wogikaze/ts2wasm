---
id: 286
title: "Classify expected negative SyntaxError tests correctly"
type: bug
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
This bucket appears 93 times and should be handled before lower-frequency failures.

Problem: Negative test262 parse/SyntaxError cases are currently counted as failures when compilation succeeds, creating a high-count false failure bucket.

## Current failure

Representative case:

```text
/home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/asi/S7.9_A4.js
```

Reason:

```text
ExpectedNegativeFailure: negative parse/SyntaxError completed successfully
```

Stderr bucket (93 occurrences):

```text
ExpectedNegativeFailure: negative parse/SyntaxError completed successfully
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

- [ ] Represent expected-negative parse outcomes explicitly in test262 result classification.
- [ ] A representative `language/asi` negative SyntaxError case is no longer reported as a failure when the expected error condition is satisfied.
- [ ] Web-ui stderr aggregation no longer reports this exact bucket as an actionable compiler failure.

Out of scope:

- Changing compiler parser behavior unless the representative case proves the parser is wrong.
- Runtime negative tests unrelated to parse/SyntaxError metadata.

## Affected paths

Expected:

- ``scripts/run/test262.py``
- ``scripts/run/reference-coverage.py``
- ``artifacts/coverage/results/test262-results.jsonl``

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
mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/asi/S7.9_A4.js
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
