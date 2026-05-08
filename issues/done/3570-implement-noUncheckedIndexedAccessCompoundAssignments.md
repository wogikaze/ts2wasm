---
id: 3570
title: "Implement Nouncheckedindexedaccesscompoundassignments"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: [5311]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage noUncheckedIndexedAccessCompoundAssignments across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this fixture tokenizes property and element update/
compound-assignment forms, then fails at the first property-access `+=`:

```text
UnsupportedSyntax: expected Semicolon, got Some(PlusEqual) at 191..193
```

Problem: this generated bucket is superseded by issue 5311, which owns the
property-access `+=` parser boundary before semantic noUncheckedIndexedAccess
diagnostics can be compared.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noUncheckedIndexedAccessCompoundAssignments.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noUncheckedIndexedAccessCompoundAssignments.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5311-parse-property-access-arithmetic-compound-assignments.md`.
Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold into existing issue 5311 for the same observable parser boundary
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing issue 5311 contains the implementation owner; this done issue contains the exact focused triage command
- [x] Evidence includes failing path, diagnostic code, source context, visible symbols, and TypeScript AST evidence
- [x] Issue 5311 acceptance covers the property-access `+=` parser boundary; later `*=` and element-access/update forms must be re-triaged after 5311 advances

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUncheckedIndexedAccessCompoundAssignments.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUncheckedIndexedAccessCompoundAssignments.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue fold.
- `cargo nextest run`; metadata-only issue fold.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded into: `issues/open/5311-parse-property-access-arithmetic-compound-assignments.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noUncheckedIndexedAccessCompoundAssignments.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### Smart triage: Triage parser syntax: noUncheckedIndexedAccessCompoundAssignments

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/noUncheckedIndexedAccessCompoundAssignments.ts`

Current compiler message:

```text
expected Semicolon, got Some(PlusEqual) at 191..193
```

Source context:

```text
 7 | stringMap.foo++;
 8 | --stringMap.foo;
 9 | stringMap.foo += 1;
10 | stringMap.foo *= 1;
11 | ++stringMap['foo'];
12 | stringMap['foo']--;
```

Compiler evidence:

```text
tokens: ok; PlusEqual token is present after property-access target stringMap.foo
ast/resolved: fail before semantic analysis with expected Semicolon, got Some(PlusEqual)
```

TypeScript oracle:

```text
diagnostics=[]
AST includes ExpressionStatement `stringMap.foo += 1;`
AST classifies `+=` as FirstCompoundAssignment
```

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noUncheckedIndexedAccessCompoundAssignments.ts --detail --no-dashboard-data
result: pass; representative path reports UnsupportedSyntax/parser-syntax for property-access PlusEqual
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noUncheckedIndexedAccessCompoundAssignments.ts
result: pass; fresh triage shows the same property-access `+=` parser boundary owned by issue 5311
date: 2026-05-08
```

Remaining risks:

- The later `stringMap.foo *= 1`, element-access updates, element-access `+=`, and noUncheckedIndexedAccess semantic checks remain hidden until issue 5311 advances.
