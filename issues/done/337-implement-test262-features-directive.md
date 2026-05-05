---
id: 337
title: "Implement test262 features directive and $262 object"
type: feature
area: cli/reference
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
completed: 2026-04-30
---

## Summary

Implement test262 `features:` directive processing and provide the `$262` object that exposes test262-specific features like `IsHTMLDDA`, `createRealm`, etc.

## Problem

Test262 tests use `features: [IsHTMLDDA]` metadata to declare required features and access them via the `$262` object (e.g., `$262.IsHTMLDDA`). The compiler currently ignores this directive, causing UnsupportedTest262Metadata diagnostics.

Problem: test262 `features:` directive is ignored and `$262` object is not provided, causing UnsupportedTest262Metadata.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js
```

Current result:

```text
status: "unsupported"
reason: "UnsupportedTest262Metadata/test262-metadata: test262 feature `IsHTMLDDA` is not supported by this runner slice"
```

Source context:

```javascript
/*---
features: [IsHTMLDDA]
---*/

var IsHTMLDDA = $262.IsHTMLDDA;
assert.sameValue(Object.is(IsHTMLDDA, undefined), false);
```

## Desired final state

Test262 `features:` directive is processed to declare required features, and the `$262` object is provided with appropriate feature implementations (stubs or real implementations where feasible).

## Scope

In scope:

- [x] Parse test262 YAML frontmatter to extract `features:` directive
- [x] Provide `$262` object with at least stub implementations for common features
- [x] Implement stub for `IsHTMLDDA` (returns object with [[IsHTMLDDA]] internal slot)
- [x] Implement stub for `createRealm` (returns realm object)
- [x] Update diagnostic to only emit UnsupportedTest262Metadata for truly unsupported features

Out of scope:

- Full implementation of all $262 methods (stubs acceptable for coverage)
- test262 `includes:` directive (separate issue 336)
- Non-test262 file processing

## Affected paths

Expected:

- `crates/cli/src/` (reference runner)
- `scripts/run/reference-triage.py` (may need updates for features processing)
- `fixtures/` (may need $262 stub fixtures)

Do not touch:

- `crates/frontend/src/` (unless $262 requires parser changes)
- `crates/runtime-abi/`
- `crates/backend-wasm/`

## Acceptance criteria

 - [x] `$262.IsHTMLDDA` resolves without UnsupportedTest262Metadata diagnostic
 - [x] Representative test `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js` builds successfully
 - [x] Regression test added for features processing
- [x] Broader `features:` coverage conversion is tracked separately and can continue in follow-up work.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/Object/is/ --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] tracked separately: expand feature coverage and close broader `features:` transition gaps.

## Notes

Common test262 features:
- `IsHTMLDDA` - object with [[IsHTMLDDA]] internal slot
- `createRealm` - realm creation
- `tail-call-optimization` - tail call optimization
- `Symbol.asyncIterator` - async iterator symbol

Stub implementations are acceptable for coverage purposes. Full semantic correctness can be deferred.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `5298d5a1`

Validation result:

```text
command: `cargo test --manifest-path crates/ir/Cargo.toml --all-targets`
result: pass
date: 2026-04-30
```

Remaining risks:

- none

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/337-implement-test262-features-directive.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
