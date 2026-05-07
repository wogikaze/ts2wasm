---
id: 1371
title: "Implement Commentsmodules"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: [5287]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed by splitting the current representative failure into a narrow
implementation-ready namespace binding issue.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsModules` with diagnostics: import-export. Fresh focused triage on
2026-05-07 shows namespace syntax is tokenized and the outer statement stream
advances, but resolver lookup cannot find the namespace value `m1` for
qualified access after the namespace declaration.

Problem: `commentsModules.ts` now fails at `m1.fooExport()` with
`UnresolvedName: unresolved name: m1`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsModules.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsModules.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Smart triage reports:

```text
error: [UnresolvedName] unresolved name: `m1` at 795..797
```

## Desired final state

This generated bucket is split into implementation-ready child issues or
superseded by existing work. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsModules.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsModules.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsModules.ts`

## Duplicate detection

No exact duplicate found by path/title/feature scan.

Related but not duplicates:

- `issues/done/399-define-typescript-parse-erase-emit-boundary.md` defines the
  TypeScript namespace parse/erase/emit boundary but does not implement this
  resolver binding.
- `issues/open/432-implement-import-export.md` is the broad import/export
  triage parent and is too broad for direct implementation.
- `issues/done/5187-lower-namespace-only-multi-section-files.md` only preserves
  namespace-only multi-section bodies before triage.
- `issues/done/5225-w0-typed-wat-writer.md` covers namespace
  qualification in `extends` clauses, not top-level value access.
- `issues/done/5244-date-timezone-formatting-policy.md`
  covers function/namespace merging, not plain namespace value binding.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: commentsModules

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/commentsModules.ts
```

Source context:

```text
39 |     }
40 | } // trailing comment module
41 | m1.fooExport();
42 | var myvar = new m1.m2.c();
43 | /** module comment of m2.m3*/
44 | namespace m2.m3 {
45 |     /** Exported class comment*/
```

Compiler evidence:

```text
tokens: ok through namespace m1, export var/function/class, nested namespace m2, and qualified uses
ast: ok; outside statements include Call(Member(Ident("m1"), "fooExport")) and New(Member(Member(Ident("m1"), "m2"), "c"))
resolved: fails in resolve_names with UnresolvedName for m1
```

Visible symbols before failure include namespace-body exports `b`, `foo`,
`fooExport`, `foo2Export`, `foo3Export`, `foo4Export`, class `c`, and binding
`myvar`, but not the namespace value `m1`.

TypeScript oracle:

```text
ok: true
diagnostics: []
hints include myvar: c and exported functions inside namespace m1
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsModules.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsModules.ts
result: UnresolvedName for namespace value m1; split to issue 5287
date: 2026-05-07
```

Remaining risks:

- Issue 5287 only targets namespace value binding for qualified access. Full
  TypeScript namespace emit, declaration comments, AMD/outFile behavior, and
  nested namespace runtime semantics may expose further blockers.
