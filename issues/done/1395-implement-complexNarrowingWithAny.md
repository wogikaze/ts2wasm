---
id: 1395
title: "Implement Complexnarrowingwithany"
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

Closed as superseded by
`issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`.

Fresh focused triage shows `complexNarrowingWithAny.ts` currently stops at the
same namespace value binding boundary already owned by issue 5287. The file
declares same-file namespaces such as `namespace import46 { export class
DefaultValueAccessor { ... } }`, then later uses qualified access
`import46.DefaultValueAccessor`; resolver lookup reports `UnresolvedName` for
`import46`.

## Problem

Reference test results originally showed 1 case failing in directory
`complexNarrowingWithAny` with diagnostics: import-export. Fresh focused triage
on 2026-05-07 reports `UnresolvedName` / `name-resolution` instead.

Problem: non-ambient namespace declarations are parsed far enough for later
qualified accesses, but the namespace identifier is not bound as a value with
exported members. The first observed failure is `import46.DefaultValueAccessor`
inside `_View_AppComponent0.injectorGetInternal`.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complexNarrowingWithAny.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complexNarrowingWithAny.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
complexNarrowingWithAny.ts: UnresolvedName for `import46`
coverage: executed=1, build_pass=0, unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Resolved evidence:

```text
[pipeline] resolve_names
...
error: [UnresolvedName] unresolved name: `import46`
```

TypeScript oracle evidence:

```text
TypeScript parses the namespace declarations and reports later strict property
initialization diagnostics inside `_View_AppComponent0`, not an unresolved
namespace error for `import46`.
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5287,
which owns binding same-file non-ambient namespaces as namespace values for
qualified member access.

After issue 5287 lands, this reference path may need fresh triage for strict
property initialization diagnostics, Angular-generated narrowing behavior, or
runtime lowering of namespace-qualified class values.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5287's namespace value binding work
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Full Angular-generated template semantics
- Strict property initialization diagnostics after namespace lookup advances

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
- [x] Existing issue 5287 covers same-file namespace value binding for qualified access
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complexNarrowingWithAny.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complexNarrowingWithAny.ts
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

- [x] superseded by: `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/complexNarrowingWithAny.ts`

## Duplicate detection

- `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`
  owns binding same-file non-ambient namespace declarations as namespace values
  so qualified accesses can resolve exported members.
- `issues/open/5187-lower-namespace-only-multi-section-files.md` is related
  but covers namespace-only multi-section preservation.
- `issues/open/5244-support-namespace-merged-function-static-properties.md` is
  related but covers function/namespace merging and exported static members.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: complexNarrowingWithAny

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/complexNarrowingWithAny.ts
```

Source shape:

```text
namespace import46 {
    export class DefaultValueAccessor {
        constructor(any){}
    }
}

...
token === import46.DefaultValueAccessor
```

Compiler evidence:

```text
tokens: ok through namespace declarations and qualified uses
ast: ok; method body contains Member(Ident("import46").DefaultValueAccessor)
resolved/lowered: UnresolvedName for `import46`
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
first diagnostics are TS2564 strict property initialization diagnostics on
class fields, showing the namespace declaration itself is accepted.
```

## Completion evidence

Commits:

- superseded by `issues/open/5287-bind-namespace-declarations-for-qualified-value-access.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complexNarrowingWithAny.ts
result: pass; reproduced namespace value UnresolvedName for `import46`
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complexNarrowingWithAny.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date: 2026-05-07
```

Remaining risks:

- Later strict property initialization diagnostics and runtime/lowering behavior
  may surface after issue 5287 resolves namespace value access.
