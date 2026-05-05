---
id: 293
title: "Parse Unicode escaped identifier parts"
type: feature
area: frontend
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
---

## Summary

Create an actionable work item from the stderr set aggregation for test262 reference coverage.
This bucket appears 15 times and should be handled before lower-frequency failures.

Problem: Identifier coverage repeatedly rejects backslash escapes inside Unicode identifier parts.

## Current failure

Representative case:

```text
/home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/identifiers/part-unicode-5.2.0-escaped.js
```

Reason:

```text
UnsupportedSyntax/feature-unsupported: [UnsupportedSyntax]
```

Stderr bucket (15 occurrences):

```text
error: [UnsupportedSyntax] unsupported character: \ at 1559..1560\n
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

- [x] Parse Unicode escape sequences in identifier parts for the representative test262 identifier case.
- [x] Invalid Unicode identifier escapes remain diagnostic-backed.
- [x] The exact `unsupported character: \` stderr bucket is reduced or removed.

Out of scope:

- Full Unicode version-table policy beyond the identifier escape path needed here.
- String literal escape handling.

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
- [x] Regenerating `artifacts/coverage/results/test262-results.jsonl` shows this bucket count reduced or removed.
- [x] If behavior changes, add or update a focused fixture or regression test for the representative case.
- [x] Update coverage artifacts or current-state only when the validation run produces new facts.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/identifiers/part-unicode-5.2.0-escaped.js
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
- updated: not applicable

Current state:

- [x] not affected
- updated: not applicable

Follow-up issues:

- [x] none
- created/updated: not applicable

## Notes

This issue was generated from exact stderr set frequency, not directory-level grouping. Start with the representative case, then rerun the aggregation to confirm the count movement.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `8ffb3189 issue-293: parse unicode identifier escapes`

Validation result:

```text
command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: cargo nextest run -p ts2wasm-frontend
result: PASS (85 passed)
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli parser
result: PASS (1 passed)
date: 2026-04-29

command: mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/identifiers/part-unicode-5.2.0-escaped.js
result: PASS; build_pass=1, semantic_pass=1, unsupported=0
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run test262 -- --category '^identifiers$' --jobs 4
result: PASS; artifacts/coverage/results/test262-results.jsonl has 268 records and the exact `unsupported character: \` bucket count is 0
date: 2026-04-29

command: cargo nextest run
result: PASS (540 passed, 4 skipped)
date: 2026-04-29

command: mise run update-issue-index -- --check
result: PASS
date: 2026-04-29

command: mise run check issues
result: PASS
date: 2026-04-29
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/293-parse-unicode-escaped-identifier-parts.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
