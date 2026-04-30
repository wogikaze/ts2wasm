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

- [ ] Parse test262 YAML frontmatter to extract `features:` directive
- [ ] Provide `$262` object with at least stub implementations for common features
- [ ] Implement stub for `IsHTMLDDA` (returns object with [[IsHTMLDDA]] internal slot)
- [ ] Implement stub for `createRealm` (returns realm object)
- [ ] Update diagnostic to only emit UnsupportedTest262Metadata for truly unsupported features

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

- [ ] `$262.IsHTMLDDA` resolves without UnsupportedTest262Metadata diagnostic
- [ ] Representative test `reference/test262/test/annexB/built-ins/Object/is/emulates-undefined.js` builds successfully
- [ ] At least 30 test262 tests with `features:` directive transition from unsupported to build_pass
- [ ] Regression test added for features processing

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

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

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

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
