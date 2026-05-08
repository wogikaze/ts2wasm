---
id: 291
title: "Provide Object global binding for test262 cases"
type: feature
area: runtime/semantics
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
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

- [x] Provide a minimal `Object` global binding for the representative admitted cases.
- [x] Cases using `Object` for the tracked arguments-object bucket compile to semantic comparison or a narrower runtime diagnostic.
- [x] The exact unresolved name `Object` stderr bucket is reduced or removed.

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

- [x] The representative case no longer emits the exact stderr bucket shown above.
- [x] Regenerating `artifacts/coverage/results/test262-results.jsonl` shows this bucket count reduced or removed.
- [x] If behavior changes, add or update a focused fixture or regression test for the representative case.
- [x] Update coverage artifacts or current-state only when the validation run produces new facts.

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

- [x] not affected
- [x] updated: none

Current state:

- [x] not affected
- [x] updated: none

Follow-up issues:

- [x] none
- [x] created/updated: none

## Notes

This issue was generated from exact stderr set frequency, not directory-level grouping. Start with the representative case, then rerun the aggregation to confirm the count movement.

Close note: the representative still depends on the separate test262 `assert`
harness binding work, but the observed unresolved identifier is now `assert`, not
`Object`. The `Object.getOwnPropertyDescriptor` operation remains a deliberate
issue-291 unsupported diagnostic in the focused fixture.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `c1db1469 issue-291: classify Object descriptor gap`
- `890408f315c4654b37290cb23d00750a20bb52a2 issue-291: close Object global binding slice`

Validation result:

```text
command: mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/10.6-12-2.js
result: pass; executed=1, blocked=1, exact unresolved `Object` bucket no longer emitted
date: 2026-04-29

command: mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/arguments-object/10.6-12-2.js --detail
result: pass; representative reports UnresolvedName:name-resolution for `assert`, not `Object`
date: 2026-04-29

command: cargo nextest run object_get_own_property_descriptor_reports_issue_291
result: pass; fixture reports the narrower issue-291 Object.getOwnPropertyDescriptor unsupported diagnostic
date: 2026-04-29

command: cargo nextest run -p ts2wasm-ir
result: pass; 21 tests passed
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: cargo nextest run
result: pass; 540 tests passed, 4 skipped
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK
date: 2026-04-29

command: mise run check issues
result: pass; issue health and index queue OK
date: 2026-04-29
```

Remaining risks:

- `Object.getOwnPropertyDescriptor` remains intentionally unsupported for this slice; broader descriptor semantics need a separate implementation issue if required.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/291-provide-object-global-binding-for-test262.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
