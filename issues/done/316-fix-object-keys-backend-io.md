---
id: 316
title: "Fix Object.keys backend-io error"
type: feature
area: harness
class: blocked
priority: P0
depends_on: [5004, 336]
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Normalize the Object.keys test262 reproduction before assigning a runtime fix.
The original issue pointed at a built-ins test path, but the current
test262 runner (now merged into `scripts/run/reference-coverage.py`) only scans `test/language/**/*.js` by default, so the
representative built-ins path is not currently selectable through that runner.

## Problem

The intended Object.keys case exists in the local reference checkout:

- `reference/test262/test/built-ins/Object/keys/15.2.3.14-3-4.js`

However, clean repository worktrees only track `reference/README.md`; the local
reference suite must be supplied separately through `TS2WASM_REFERENCE_ROOT`.
Even with that reference checkout, the current test262 runner does not scan
`built-ins` tests, so this issue cannot yet be validated with the documented
focused runner command.

## Current status

Direct build with the local reference checkout on 2026-04-30:

```sh
cargo run -q -- build /home/wogikaze/wgkz/ts2wasm/reference/test262/test/built-ins/Object/keys/15.2.3.14-3-4.js -o /tmp/object-keys.wasm --host-deny
```

Result:

```text
error: [UnresolvedName] unresolved name: `assert` at 946..952
```

Focused test262 runner attempt on 2026-04-30:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference \
  python3 scripts/run/reference-coverage.py test262 --jsonl \
  --path-filter built-ins/Object/keys/15.2.3.14-3-4.js \
  --jobs 1
```

Result:

```text
ERROR: --path-filter selected no files: built-ins/Object/keys/15.2.3.14-3-4.js
```

## Desired final state

The Object.keys case has a valid harness-backed reproduction path before a
runtime/backend fix is assigned. Once the runner can execute the built-ins case,
reclassify or split a child implementation issue with exact current failure
evidence.

## Scope

In scope:

- [ ] Establish a harness-backed reproduction for the Object.keys built-ins
      case.
- [ ] Reconfirm whether the current failure is `BackendIo`, `UnresolvedName`,
      semantic mismatch, or unsupported.
- [ ] Split a child implementation issue only after the failure mode is known.

Out of scope:

- Full Object.keys implementation for all edge cases beyond the specific test case
- Other Object built-in methods

## Affected paths

Expected for the eventual implementation child issue:

- `crates/ir/src/lowered/program_builtins.rs`
- `crates/backend-wasm/src/runtime_builder.rs`
- `crates/backend-wasm/src/runtime_arrays_objects.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/runtime_fn_impl.rs`
- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/cli/tests/`

Do not touch:

- Other Object built-in implementations unless directly related to keys

## Acceptance criteria

- [ ] The Object.keys built-ins case is selectable by the focused reference
      runner or an equivalent documented harness command.
- [ ] The current failure mode is recorded with exact command output.
- [ ] If the current failure is still a product bug, a child issue owns the
      smallest implementation slice.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
TS2WASM_REFERENCE_ROOT=/home/wogikaze/wgkz/ts2wasm/reference python3 scripts/run/reference-coverage.py test262 --jsonl --path-filter built-ins/Object/keys/15.2.3.14-3-4.js --jobs 1
cargo run -q -- build /home/wogikaze/wgkz/ts2wasm/reference/test262/test/built-ins/Object/keys/15.2.3.14-3-4.js -o /tmp/object-keys.wasm --host-deny
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` if Object built-in ABI changes

Current state:

- [ ] not affected

Follow-up issues:

- [ ] create an implementation child after harness-backed reproduction exists

## Notes

**Updated 2026-05-02**: This is now classified as a harness infrastructure issue, not a runtime bug.

- Object.keys runtime implementation is complete: `RuntimeFn::ObjectKeys` exists with WAT emitter and passes `build_smoke`.
- The remaining blocker is test262 includes directive (`$ERROR`, `assert` harness helpers) not being resolved — tracked by issue 336.
- Once 336 is resolved (test262 includes support), this issue can be re-validated with the full test262 built-ins scan.

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
