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
status: done
completed: 2026-04-29
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

- [x] Represent expected-negative parse outcomes explicitly in test262 result classification.
- [x] A representative `language/asi` negative SyntaxError case is no longer reported as a failure when the expected error condition is satisfied.
- [x] Web-ui stderr aggregation no longer reports this exact bucket as an actionable compiler failure.

Out of scope:

- Changing compiler parser behavior unless the representative case proves the parser is wrong.
- Runtime negative tests unrelated to parse/SyntaxError metadata.

## Affected paths

Expected:

- ``scripts/run/reference-coverage.py``
- ``scripts/run/reference-coverage.py``
- ``artifacts/coverage/results/test262-results.jsonl``

Do not touch:

- unrelated web-ui files
- unrelated issue files

## Acceptance criteria

- [x] The representative case no longer emits the exact stderr bucket shown above.
- [x] Regenerating representative test262 classification shows this bucket removed for the case.
- [x] The focused representative regression is covered by the `process_one_test` validation and `reference-coverage` path-filter validation.
- [x] Update coverage artifacts or current-state only when the validation run produces new facts.

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

- [x] not affected
- not updated

Current state:

- [x] not affected
- not updated

Follow-up issues:

- [x] none
- not created

## Notes

This issue was generated from exact stderr set frequency, not directory-level grouping. Start with the representative case, then rerun the aggregation to confirm the count movement.

## Completion evidence

Commits:

- `b649d012` (`issue-286: classify negative syntax outcomes`)

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: python3 -m py_compile scripts/run/reference-coverage.py scripts/run/reference-coverage.py
result: PASS
date: 2026-04-29

command: TS2WASM_TEST262_ROOT=/home/wogikaze/wgkz/ts2wasm/reference/test262 python3 - <<'PY' ... process_one_test(S7.9_A4.js) ... PY
result: PASS; status=unsupported; tracking=feature:negative-parse-syntaxerror; old_bucket_present=false
date: 2026-04-29

command: mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/asi/S7.9_A4.js
result: PASS; fail=0; unsupported_diagcodes=ExpectedNegativeSyntax:1; unsupported_features=negative-parse-syntaxerror:1
date: 2026-04-29

command: mise run check scripts
result: PASS
date: 2026-04-29

command: mise run update-issue-index -- --check
result: PASS before issue close
date: 2026-04-29

command: mise run check issues
result: PASS before issue close
date: 2026-04-29

command: mise run check agent-state
result: PASS
date: 2026-04-29

command: cargo nextest run
result: PASS; 537 tests run, 537 passed, 4 skipped
date: 2026-04-29
```

Remaining risks:

- none
