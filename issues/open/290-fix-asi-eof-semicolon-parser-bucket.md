---
id: 290
title: "Fix ASI EOF semicolon parser bucket"
type: bug
area: frontend
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
status: done
completed: 2026-04-29
---

## Summary

Create an actionable work item from the stderr set aggregation for test262 reference coverage.
This bucket appears 36 times and should be handled before lower-frequency failures.

Problem: ASI coverage repeatedly reports EOF where the parser still expects an explicit semicolon.

## Current failure

Representative case:

```text
/home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/asi/S7.9_A10_T1.js
```

Reason:

```text
UnsupportedSyntax/feature-unsupported: [UnsupportedSyntax]
```

Stderr bucket (36 occurrences):

```text
error: [UnsupportedSyntax] expected Semicolon, got None\n
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

- [x] Handle the representative automatic semicolon insertion case at EOF.
- [x] Keep invalid ASI cases diagnostic-backed.
- [x] The exact `expected Semicolon, got None` stderr bucket is reduced or removed in regenerated test262 results.

Out of scope:

- All ASI edge cases beyond the representative EOF bucket.
- Negative SyntaxError classification, tracked separately.

## Affected paths

Expected:

- ``crates/frontend/src/``
- ``crates/cli/tests/``
- ``fixtures/``

Do not touch:

- unrelated web-ui files
- unrelated issue files

## Acceptance criteria

- [x] The representative case no longer emits the exact stderr bucket shown above.
- [x] Regenerating the representative reference coverage result shows this bucket removed for the case.
- [x] If behavior changes, add or update a focused fixture or regression test for the representative case.
- [x] Update coverage artifacts or current-state only when the validation run produces new facts.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/asi/S7.9_A10_T1.js
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

- `0c298194` (`issue-290: accept EOF expression semicolons`)

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cargo nextest run -p ts2wasm-frontend
result: PASS; 83 tests run, 83 passed
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli parser
result: NO TESTS SELECTED; nextest exit 4 because the filter matches zero CLI tests
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli
result: PASS; 343 tests run, 343 passed, 4 skipped
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter reference/test262/test/language/asi/S7.9_A10_T1.js --detail
result: PASS; build_pass=1, semantic_pass=1, unsupported=0; representative file no longer reports UnsupportedSyntax/parser-syntax
date: 2026-04-29

command: cargo nextest run
result: PASS; 536 tests run, 536 passed, 4 skipped
date: 2026-04-29

command: mise run update-issue-index -- --check
result: PASS
date: 2026-04-29

command: mise run check issues
result: PASS after generating the local ignored test262-results.jsonl artifact required by unrelated open issue path checks
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/290-fix-asi-eof-semicolon-parser-bucket.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
