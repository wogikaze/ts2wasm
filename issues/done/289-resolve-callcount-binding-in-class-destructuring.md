---
id: 289
title: "Resolve callCount binding in class destructuring tests"
type: bug
area: frontend/ir
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-04-29
updated: 2026-04-29
completed: 2026-04-29
status: done
---

## Summary

Create an actionable work item from the stderr set aggregation for test262 reference coverage.
This bucket appears 60 times and should be handled before lower-frequency failures.

Problem: Class destructuring method tests repeatedly lose or fail to resolve the `callCount` local used to observe evaluation count.

## Current failure

Representative case:

```text
/home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/expressions/class/dstr/meth-ary-name-iter-val.js
```

Reason:

```text
UnresolvedName/feature-resolution: [UnresolvedName]
```

Stderr bucket (60 occurrences):

```text
error: [UnresolvedName] unresolved name: `callCount`\n
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

- [x] Triage the representative class destructuring test and identify where `callCount` leaves scope resolution.
- [x] Preserve or resolve the side-effect counter binding for the supported subset.
- [x] The representative case no longer reports unresolved name `callCount`.

Out of scope:

- All class destructuring semantics beyond the side-effect local binding bucket.
- Iterator protocol completeness unless required by the representative case.

## Affected paths

Expected:

- ``crates/frontend/src/``
- ``crates/ir/src/``
- ``crates/backend-wasm/src/``

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
mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/expressions/class/dstr/meth-ary-name-iter-val.js
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
- [x] updated: n/a

Current state:

- [x] not affected
- [x] updated: n/a

Follow-up issues:

- [x] none
- [x] created/updated: n/a

## Notes

This issue was generated from exact stderr set frequency, not directory-level grouping. Start with the representative case, then rerun the aggregation to confirm the count movement.

### Progress note: 2026-04-29 child-289-callcount-binding

The first attempted close identified the likely implementation gap: class methods are lowered as separate functions, and references to outer locals such as `callCount` are not represented as span-preserving lexical captures for class elements. A reduced source with:

```js
var callCount = 0;
var C = class {
  method([x, y, z]) {
    callCount = callCount + 1;
  }
};
new C().method([1, 2, 3]);
```

reaches the `callCount` lowering path. However, the attempted narrow diagnostic was rejected because the IR `ResolvedExpr::Ident` / `ResolvedStmt::Assign` forms do not currently carry source spans, so the new source-origin diagnostic had `span: None`. That change was reverted.

Current close requirement: implement a span-preserving class-method outer-local capture path, or otherwise produce a source diagnostic with a real span. Do not close this issue on an unspanned source diagnostic. In this worktree, the raw representative reference command still stops earlier at unresolved `assert`, so exact `callCount` bucket movement must be verified either after the issue-288/assert prerequisite lands or with an accepted reduced, source-span-backed representative.

### Progress note: 2026-04-29 child-019dd968 issue-289

Implemented a narrow source-span-preserving `issue-289` diagnostic for class methods that reference lexical outer locals, so the reduced `callCount` method case no longer falls through to the later unspanned `UnresolvedName` path. The diagnostic deliberately excludes the declaring class name, preserving existing supported static private method calls such as `C.#m()`.

Validation:

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(class) or test(destructuring) or test(node_diff)'
result: pass, 51 passed

mise run reference-coverage -- test262 --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/expressions/class/dstr/meth-ary-name-iter-val.js
result: pass, executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=class:1

mise run update-issue-index -- --check
result: pass

TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run test262 -- --path-filter /home/wogikaze/wgkz/ts2wasm/reference/test262/test/language/expressions/class/dstr/meth-ary-name-iter-val.js --jobs 1
result: pass, Total=1, Unsupported=1, generated ignored artifacts/coverage/results/test262-results.jsonl for issue-health path checks

mise run check issues
result: pass
```

Remaining scope: class-method lexical capture environments are still not implemented, so the issue remains open as PROGRESS rather than DONE.

### Progress note: 2026-04-29 child-019dda15 issue-289

Implemented the smallest validated class-method lexical capture slice: class
methods that read immutable outer locals now carry those locals as hidden
parameters at known class-method call sites. Added
`fixtures/core-semantics/class-method-immutable-outer-capture.ts` and an IR
lowering regression proving the hidden capture argument is appended.

Mutable outer local captures, including the original `callCount = callCount + 1`
bucket, remain unsupported with a spanned `issue-289` diagnostic because the
current closure/class ABI has no shared mutable environment cell. Split that
remaining work to
`issues/open/301-implement-mutable-class-method-outer-environment-cells.md`.

Validation:

```text
cargo nextest run -E 'test(class_method_outer_local_capture_reports_spanned_issue_289) or test(lowering_passes_immutable_class_method_outer_local_capture) or test(this_receiver_method_fixtures_match_node_output_under_iwasm)'
result: pass, 3 passed
```

## Completion evidence

Commits:

- `cbb9a351` (`issue-289: close callcount binding bucket`)

Validation result:

```text
date: 2026-04-29

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run reference-coverage -- test262 --path-filter language/expressions/class/dstr/meth-ary-name-iter-val.js --detail
result: pass; executed=1, blocked=1. The representative no longer emits `error: [UnresolvedName] unresolved name: `callCount``.

command: TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference mise run test262 -- --path-filter language/expressions/class/dstr/meth-ary-name-iter-val.js --jobs 1 --verbose
result: pass; Total=1, Unsupported=1. JSONL stderr is `error: [UnsupportedSyntax] issue-211: method `method` requires an identifier receiver at 4297..4322`; no `callCount` unresolved-name bucket remains.

command: python3 aggregation over artifacts/coverage/results/test262-results.jsonl
result: exact `error: [UnresolvedName] unresolved name: `callCount`\n` bucket count is 0; most common stderr is the issue-211 identifier-receiver diagnostic for the representative.

command: cargo fmt --all --check
result: pass

command: cargo nextest run -p ts2wasm-cli -E 'test(class_method) or test(destructuring) or test(this_receiver_method_fixtures_match_node_output_under_iwasm)'
result: pass; 17 passed
```

Remaining risks:

- The representative is still not semantically passing; it is now stopped by issue-211 receiver support rather than issue-289 `callCount` binding.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/289-resolve-callcount-binding-in-class-destructuring.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
