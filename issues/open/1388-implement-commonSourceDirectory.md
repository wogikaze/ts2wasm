---
id: 1388
title: "Implement Commonsourcedirectory"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: [5187]
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1388.

## Summary

Closed as superseded by
`issues/open/5187-lower-namespace-only-multi-section-files.md`.

Fresh focused triage shows this bucket currently stops at declaration-only /
namespace-only `.d.ts` multi-section handling, not common source directory
calculation itself.

## Problem

Reference test results originally showed 2 cases failing in directory
`commonSourceDirectory` with diagnostics: import-export. Fresh focused triage on
2026-05-07 shows different current diagnostics for the two paths, but both are
in the same multi-section declaration-file preservation family.

Problem: `commonSourceDirectory_dts.ts` declares `y` in a virtual
`/app/lib/bar.d.ts` section and uses it from `/app/src/index.ts`, but current
name resolution reports `UnresolvedName` for `y`. `commonSourceDirectory.ts`
contains a virtual `/types/bar.d.ts` section with `declare module "bar" { ... }`
and currently reports that namespace-only declarations in the section cannot be
lowered.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDirectory_dts.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDirectory.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDirectory_dts.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDirectory.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
commonSourceDirectory_dts.ts: UnresolvedName for `y` from a virtual .d.ts section
commonSourceDirectory.ts: UnsupportedRuntimeSubset for namespace-only declarations in `/types/bar.d.ts`
coverage: build_pass=0, unsupported=1 for each focused path
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5187.
After declaration-only `.d.ts` sections are preserved or diagnosed precisely,
these reference paths may need fresh triage for `tsconfig.json` sections,
sourceDir/outDir/sourceMap behavior, or module specifier behavior.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing multi-section declaration-file issue
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
- [x] Existing issue 5187 covers declaration-only `.d.ts` multi-section handling
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference paths and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDirectory_dts.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDirectory.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDirectory_dts.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDirectory.ts
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

- [x] superseded by: `issues/open/5187-lower-namespace-only-multi-section-files.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commonSourceDirectory_dts.ts`
- `reference/typescript/tests/cases/compiler/commonSourceDirectory.ts`

## Duplicate detection

- `issues/open/5187-lower-namespace-only-multi-section-files.md` owns keeping
  namespace-only/declaration-only multi-section `.d.ts` bodies observable enough
  for the next namespace/scope diagnostic.
- `issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`
  is a later adjacent risk for the `tsconfig.json` sections in these files, but
  focused triage reports declaration-file handling first.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage name resolution: commonSourceDirectory dts

- Issue class: triage-needed
- Feature label: name-resolution
- Diagnostic: UnresolvedName / resolver-symbol
- Path: reference/typescript/tests/cases/compiler/commonSourceDirectory_dts.ts

### Smart triage: Triage runtime subset: commonSourceDirectory

- Issue class: triage-needed
- Feature label: runtime-subset
- Diagnostic: UnsupportedRuntimeSubset / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/commonSourceDirectory.ts
```

Source context:

```text
// @filename: /app/lib/bar.d.ts
declare const y: number;

// @filename: /app/src/index.ts
/// <reference path="../lib/bar.d.ts" preserve="true" />
export const x = y;

// @filename: /types/bar.d.ts
declare module "bar" {
    export const y = 0;
}
```

Compiler evidence:

```text
commonSourceDirectory_dts.ts stack trace: UnresolvedName unresolved name `y`
commonSourceDirectory.ts stack trace: multi-section section `/types/bar.d.ts` contains namespace-only declarations; namespace lowering is not implemented
tokens: ok through declaration-only .d.ts sections and later tsconfig.json sections
ast/resolved dumps also show later tsconfig.json object bodies, but they are not the first build diagnostic
```

TypeScript oracle:

```text
TypeScript recognizes the declaration sections enough to report later
tsconfig/sourceDir/module diagnostics. The current compiler stops before those
checks because declaration-only virtual sections are not preserved or lowered
for cross-section lookup.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDirectory_dts.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonSourceDirectory.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDirectory_dts.ts
result: unresolved `y` from virtual .d.ts section; superseded by issue 5187
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonSourceDirectory.ts
result: namespace-only `/types/bar.d.ts` multi-section boundary; superseded by issue 5187
date: 2026-05-07
```

Remaining risks:

- After issue 5187 advances these paths, fresh triage may expose `tsconfig.json`
  section handling, sourceDir/outDir/sourceMap behavior, or bare package
  specifier/module-resolution blockers.
