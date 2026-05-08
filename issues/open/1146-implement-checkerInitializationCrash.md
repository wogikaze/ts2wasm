---
id: 1146
title: "Implement Checkerinitializationcrash"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1146.

## Summary

Triage checkerInitializationCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `checkerInitializationCrash` with diagnostics: parser-syntax. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: checkerInitializationCrash has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkerInitializationCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkerInitializationCrash.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing ambient declaration erasure boundary
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
- [x] This closure records an exact `reference-triage` command
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issue/docs evidence names the exact diagnostic behavior

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/checkerInitializationCrash.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/checkerInitializationCrash.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by `issues/done/400-implement-ambient-declaration-erasure-boundary.md` and `issues/done/5044-frontend-ambient-erasure.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/checkerInitializationCrash.ts`

## Duplicate detection

Fresh duplicate and policy review found that the current blocker is the
already-defined issue-400 ambient declaration boundary, not a new checker
initialization or module-resolution implementation slice.

Superseded by:

- `issues/done/400-implement-ambient-declaration-erasure-boundary.md`
- `issues/done/5044-frontend-ambient-erasure.md`

Rationale: this representative stops at the first `declare global { ... }`.
`docs/language-reference/typescript-features.md` classifies `declare global`
as "Rejected (runtime impact)" with `UnsupportedTypeScriptSyntax`. The parser
currently emits the expected issue-400 diagnostic for that construct.

Potential later blockers are already split or remain outside this first
observable blocker: `export as namespace` is tracked by
`issues/open/5231-parse-export-as-namespace-declarations.md`; external package
module resolution and declaration-file checker behavior are not reached while
`declare global` is rejected.

## Smart triage

Fresh triage shows the generated parser-syntax bucket is not an
implementation-ready checker-initialization crash slice. The compiler stops at
the ambient global declaration boundary in the first virtual declaration file.

### Smart triage: checkerInitializationCrash

- Issue class: `triage-needed`
- Feature label: `parser-syntax`
- Diagnostic: `UnsupportedTypeScriptSyntax` / `unsupported-feature-boundary`
- Current compiler message: `issue-400: ambient global declarations are not supported in this erasure slice`
- Path: `reference/typescript/tests/cases/compiler/checkerInitializationCrash.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkerInitializationCrash.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkerInitializationCrash.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
unsupported_features=parser-syntax:1
```

Source context:

```ts
import * as react from 'react';
declare global {
  namespace FullCalendarVDom {
    export import VNode = react.ReactNode;
  }
}

export default class FullCalendar {
}
```

Compiler evidence:

```text
tokens: ok; declare global block tokens are present
ast: issue-400 ambient global declarations are not supported at global span 194..200
resolved/lowered: same parser/ambient boundary diagnostic
TypeScript oracle: ModuleDeclaration declare global, plus unresolved react/preact/package imports and export-as-namespace diagnostics
```

Policy evidence:

```text
docs/language-reference/typescript-features.md:
declare global { ... } -> Rejected; cannot be safely erased -> UnsupportedTypeScriptSyntax
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- filled by closing commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/checkerInitializationCrash.ts --detail --no-dashboard-data
result: pass; reproduced UnsupportedTypeScriptSyntax/parser-syntax blocker
date: 2026-05-06

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/checkerInitializationCrash.ts
result: pass; reproduced issue-400 declare global boundary, superseded by done issue 400/5044
date: 2026-05-06
```

Remaining risks:

- If future policy allows erasing limited `declare global` declarations, this fixture can be revisited under a new design issue.
- Once `declare global` advances, this representative may next expose `export import` aliasing, `export as namespace` (issue 5231), `export =`, or virtual package module-resolution work.
