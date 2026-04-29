---
id: 291
title: "Provide Object global binding for test262 cases"
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Create an actionable work item from the stderr set aggregation for test262 reference coverage.
This bucket appears 30 times and should be handled before lower-frequency failures.

Problem: Several test262 cases fail before semantic comparison because the global `Object` binding is unresolved.

## Current failure

Representative case:

```text
/home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/10.6-12-2.js
```

Reason:

```text
UnresolvedName/feature-resolution: [UnresolvedName]
```

Stderr bucket (30 occurrences):

```text
error: [UnresolvedName] unresolved name: `Object`\n
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

- [ ] Provide a minimal `Object` global binding for the representative admitted cases.
- [ ] Cases using `Object` for the tracked arguments-object bucket compile to semantic comparison or a narrower runtime diagnostic.
- [ ] The exact unresolved name `Object` stderr bucket is reduced or removed.

Out of scope:

- Complete Object constructor/prototype API coverage beyond the calls required by the representative bucket.
- Other global builtins such as Array, Map, or Symbol.

## Affected paths

Expected:

- ``crates/ir/src/``
- ``crates/backend-wasm/src/``
- ``crates/runtime-abi/src/``
- ``fixtures/``

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
mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/10.6-12-2.js
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
