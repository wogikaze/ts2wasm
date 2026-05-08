---
id: 1203
title: "Implement Classextendsinterfacethatextendsclasswithprivates"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1203.

## Summary

Triage classExtendsInterfaceThatExtendsClassWithPrivates across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results originally showed 1 stale generated parser-syntax bucket
for `classExtendsInterfaceThatExtendsClassWithPrivates`. Fresh smart triage now
shows the parser accepts the file and ts2wasm build-passes; TypeScript reports a
semantic TS2420 private-member-origin mismatch.

Problem: classExtendsInterfaceThatExtendsClassWithPrivates was a stale
generated bucket and needed smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5316-report-class-implements-interface-private-member-mismatch.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated 2026-05-07:

```sh
TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts
```

Result:

```text
ts2wasm: BuildPass
feature_label: build-pass
diagnostic: BuildPass / pass
visible symbols: class C, class D2
TypeScript oracle:
TS2420 Class 'D2' incorrectly implements interface 'I'.
  Types have separate declarations of a private property 'x'.
```

Current compiler evidence:

- Tokens include `interface I extends C`, `class D2 implements I`, and both
  `private x` declarations.
- AST/resolved IR erase the interface and typed private fields, retaining only
  runtime methods for `C` and `D2`.
- Reference coverage reports `build_pass=1`, `semantic_pass=0` for this single
  file because semantic comparison is not enabled for the window.

Duplicate review:

- `issues/open/5158-report-interface-private-member-clash.md` is related but
  narrower to `interface Z extends X, Y` TS2320 private-member clashes.
- `issues/open/5315-report-class-extends-interface-diagnostics.md` is related
  but narrower to TS2689 for `class ... extends Interface`.
- No exact open/done owner was found for the TS2420 `class implements interface`
  private-member-origin mismatch.

## Completion evidence

This generated bucket was split and closed as superseded by issue 5316.

Commits:

- this commit

Validation result:

```text
command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts
result: pass; stale parser-syntax bucket reclassified as build-pass with TypeScript TS2420 semantic mismatch
date: 2026-05-07

command: TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classExtendsInterfaceThatExtendsClassWithPrivates1.ts --detail --no-dashboard-data
result: pass; single-file window reports build_pass=1 and semantic_pass=0
date: 2026-05-07
```

Remaining risks:

- Implementation remains open in issue 5316.
