---
id: 1265
title: "Implement Collectionpatternnoerror"
type: spike
area: runtime/builtins
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
---

## Summary

Closed after splitting the current blocker to
`issues/open/5272-support-generic-return-interface-method-receivers.md`. Fresh
triage shows this bucket now parses and reaches lowering, where generic
function return information is not preserved enough to resolve an interface
method receiver.

## Problem

Reference test results originally showed 1 case failing in directory
`collectionPatternNoError` with diagnostics: parser-syntax. Fresh coverage and
triage show tokens and AST succeed, then lowering reports `UnsupportedSyntax`
for `messageList.methodOnMessageList()`.

Problem: the local `messageList` is initialized from
`fetchMsg(this.messageList)`, whose TypeScript type is `U extends
MessageList<T>`, but the compiler falls through to `issue-211: unknown receiver
class for method methodOnMessageList`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collectionPatternNoError.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collectionPatternNoError.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm related issue-211 receiver issues do not exactly own this generic return shape
- [x] Split one observable behavior into child issue 5272
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- `fixtures/`
- `scripts/run/reference-triage.py`

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed and the exact behavior is split to 5272
- [x] Child issue 5272 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collectionPatternNoError.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collectionPatternNoError.ts
```

Not run:

- `cargo fmt --all --check` (not run; issue metadata only)
- `cargo nextest run` (not run; issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5272-support-generic-return-interface-method-receivers.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collectionPatternNoError.ts`

Source context:

```ts
class DataProvider<T extends Message, U extends MessageList<T>> {
  fetch() {
    const messageList = fetchMsg(this.messageList);
    messageList.methodOnMessageList();
  }
}
```

## Duplicate detection

- `issues/done/5222-parse-ambient-generic-variable-type-annotations.md`
  is related but not exact: it handles locals directly annotated with interface
  types, while this bucket's current blocker is a local inferred from a generic
  function return constrained to an interface.
- `issues/done/5234-w0-implement-host-deny-and-auditable-e2e-manifest-verification.md` is
  related but not exact: it handles array-shaped parameter annotations.
- `issues/open/5261-report-class-typed-missing-instance-method-calls.md` is
  related but not exact: it handles class-typed ambient locals with missing
  instance methods.
- No exact implementation-ready issue owned the generic return interface
  receiver shape, so this bucket was split to issue 5272.

## Smart triage

Fresh commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collectionPatternNoError.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collectionPatternNoError.ts
```

Observed result on 2026-05-06:

```text
coverage: unsupported=1
unsupported_diagcodes: UnsupportedSyntax:1
unsupported_features: unknown-unsupported:1

Diagnostic: UnsupportedSyntax
Message: issue-211: unknown receiver class for method `methodOnMessageList` at 619..652
Source: messageList.methodOnMessageList();
tokens: ok
AST: ok; ClassDecl DataProvider contains constructor parameter property assignments and fetch()
resolved/lowered: issue-211 unknown receiver class for method methodOnMessageList
TypeScript oracle: ok, no diagnostics; binding messageList has type U
Child issue: 5272
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Split to `issues/open/5272-support-generic-return-interface-method-receivers.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collectionPatternNoError.ts
result: pass; reproduced generic return interface receiver issue-211 and split child issue 5272
date: 2026-05-06
```

Remaining risks:

- none; remaining implementation work is tracked by issue 5272
