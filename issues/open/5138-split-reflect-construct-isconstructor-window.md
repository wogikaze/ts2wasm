---
id: 5138
title: "Split Reflect.construct isConstructor reference window"
type: spike
area: runtime/builtins
class: design-ready
priority: P1
depends_on: []
blocks: [068]
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Decide and split the `Reflect.construct` / test262 `isConstructor.js` support needed by Annex B String HTML-method `not-a-constructor` cases.

Problem: `reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js` currently fails with `UnresolvedName` for `Reflect`, so the old unsupported-expression bucket is hiding a concrete Reflect global / constructor-probe gap.

## Problem

The Annex B String HTML-method `not-a-constructor` reference window uses the test262 `isConstructor.js` helper, which calls `Reflect.construct`. The current compiler reaches name resolution and fails before the String method diagnostic or runtime behavior can be evaluated.

## Current failure

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js
```

Observed triage result:

```text
Issue class: triage-needed
Feature label: name-resolution
Diagnostic: UnresolvedName / resolver-symbol
Path: reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js
Failure: unresolved name: Reflect at 1548..1555
Visible symbols before failure: print, NaN, Infinity, $262, $ERROR, $DONOTEVALUATE, assert, isConstructor
```

Related reference window:

- `reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/big/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/blink/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/bold/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fixed/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontcolor/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/fontsize/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/italics/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/link/not-a-constructor.js`
- `reference/test262/test/annexB/built-ins/String/prototype/small/not-a-constructor.js`

## Desired final state

The reference window is represented by one or more executable child issues that either implement the minimal `Reflect.construct` / constructor-probe support needed by `isConstructor.js`, or define a precise issue-linked diagnostic policy for unsupported Reflect builtins.

## Scope

In scope:

- [ ] Decide whether the next slice should support `Reflect` as a global binding, implement minimal `Reflect.construct`, or emit a precise unsupported builtin diagnostic.
- [ ] Split implementation work into child issues with exact reference paths and expected diagnostic/stdout behavior.
- [ ] Update the stale `Reflect` feature mapping if issue 5025 is not the correct owner for this reference window.

Out of scope:

- Full Reflect API implementation.
- Implementing Annex B String HTML methods themselves.
- Broad name-resolution refactors unrelated to global builtin recognition.

## Affected paths

Expected:

- `crates/compiler/src/test262_preprocessor.rs`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `fixtures/builtins-and-io/`
- `issues/open/`

Do not touch:

- unrelated parser syntax work
- unrelated String builtin behavior outside the not-a-constructor reference window

## Acceptance criteria

- [ ] The selected child issue names the exact `anchor/not-a-constructor.js` reproduction command and expected `Reflect` diagnostic or pass behavior.
- [ ] The selected child issue covers at least the Annex B String HTML-method `not-a-constructor` reference window listed in this issue.
- [ ] The selected child issue records whether `crates/compiler/src/test262_preprocessor.rs` should continue mapping `Reflect` / `Reflect.construct` to issue 5025.
- [ ] Issue 068 remains closed as a superseded generated bucket.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/String/prototype/anchor/not-a-constructor.js --detail
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] create implementation child issue(s) after the Reflect support decision
