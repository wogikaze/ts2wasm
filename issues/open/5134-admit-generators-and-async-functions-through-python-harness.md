---
id: 5134
title: "Admit generators and async-functions features through Python test262 harness"
type: spike
area: scripts
class: design-ready
priority: P2
depends_on: [416]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

The Python test262 harness rejects 2054 test cases at the metadata level because
their `features:` include `generators`, `async-functions`, etc., which are not
in `SUPPORTED_FEATURES`. The Rust preprocessor already admits `generators` (via
issue-401's parser support). This spike adds the missing features to the Python
harness whitelist so tests reach the Rust compiler.

## Problem

Problem: The Python test262 harness rejects generator and async-function metadata before these cases can reach compiler diagnostics.

The Python harness at `scripts/lib/test262_harness.py` line 27 defines
`SUPPORTED_FEATURES = ("class",)`. Any test262 test case with a `features:`
value not in this tuple is rejected with "test262 feature `{feature}` is not
supported by this runner slice" before it ever reaches the Rust compiler.

The Rust preprocessor (`crates/compiler/src/test262_preprocessor.rs`) has a
more permissive feature list via `KNOWN_FEATURES` and already admits
`generators` (mapped to issue-401, parser-level implementation done). But the
Python harness does not forward these features, creating a false-negative gap.

Representative failure:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/yield/star-iterable-return-emulates-undefined-throws-when-called.js
```

Produces:

```text
error: [UnsupportedSyntax] UnsupportedTest262Metadata/test262-metadata: test262 feature `generators` is not supported by this runner slice
```

This error comes from the Python harness, not the Rust compiler. The same
pattern applies to `async-functions` (used by `async function`, `async () =>`,
etc.) which has no tracking issue at all.

## Desired final state

Test262 test cases with features `generators` and `async-functions` are no
longer rejected by the Python harness. They reach the Rust compiler, which
either compiles them (generator syntax already works per issue-401) or produces
a more precise downstream diagnostic (async function body not yet lowered).

## Scope

In scope:

- [ ] Add `"generators"` and `"async-functions"` to `SUPPORTED_FEATURES` in `scripts/lib/test262_harness.py`
- [ ] Verify representative test reaches the Rust compiler with a new diagnostic
- [ ] Run `mise run reference-triage -- test262 <representative-case>` to confirm no more Python-level rejection
- [ ] Update coverage artifacts and evidence

Out of scope:

- Full async/await lowering in the compiler
- Generator function execution beyond parser-level support

## Affected paths

Expected:

- `scripts/lib/test262_harness.py` (the `SUPPORTED_FEATURES` tuple)
- `artifacts/coverage/results/*` (regeneration)

Do not touch:

- `crates/` (compiler-level changes)
- Unrelated issue/dashboard files

## Acceptance criteria

- [ ] `generators` and `async-functions` added to `SUPPORTED_FEATURES`
- [ ] Representative `generators` test no longer fails with Python harness rejection
- [ ] Representative `async-functions` test no longer fails with Python harness rejection
- [ ] Coverage artifacts regenerated and verified

## Validation

Required commands:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/yield/star-iterable-return-emulates-undefined-throws-when-called.js
```

The test should no longer produce "UnsupportedTest262Metadata" with
"generators" in the message. Instead it should either compile or produce a
downstream diagnostic.

```sh
mise run reference-triage -- test262 reference/test262/test/language/statements/async-function/await-as-binding-identifier.js
```

Same verification: no Python-level feature rejection.

```sh
cargo fmt --all --check
cargo nextest run
mise run update-issue-index -- --check
mise run check issues
```

## Notes

The Rust preprocessor already handles `generators` in `KNOWN_FEATURES`:
it passes through silently because the feature is known (mapped to issue-401).
Adding `generators` to the Python harness whitelist simply aligns the two
layers.

`async-functions` is not in `KNOWN_FEATURES` or handled by the Rust preprocessor
match arms, so it will fall through to the "unknown feature" rejection path in
the Rust preprocessor. That is acceptable — the Rust error will be more precise
than the Python one, and will include the right tracking issue ID.
