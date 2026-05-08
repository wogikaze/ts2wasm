---
id: 1372
title: "Implement Commentsmultimodulemultifile"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1372.

## Summary

Closed as superseded by `issues/open/232-resolve-local-relative-es-module-graph.md`.

Fresh triage shows this generated bucket now reaches the completed issue-232
module graph diagnostic for a non-local specifier,
`commentsMultiModuleMultiFile_0`.

## Problem

Reference test results originally showed 1 case failing in directory
`commentsMultiModuleMultiFile` with diagnostics: import-export. Fresh focused
triage on 2026-05-07 shows parsing advances through the virtual sections,
namespace exports, and `new multiM.*` expressions, then module graph validation
rejects the non-local `require('commentsMultiModuleMultiFile_0')` specifier.

Problem: `commentsMultiModuleMultiFile.ts` is not a standalone implementation
order; its current first blocker is the intentional issue-232 unsupported
non-local module specifier diagnostic.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsMultiModuleMultiFile.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsMultiModuleMultiFile.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Resolved dump reports:

```text
error: [UnsupportedModule] issue-232: unsupported non-local module specifier `commentsMultiModuleMultiFile_0`; package resolution, import maps, and absolute specifiers are not implemented at 539..571
```

## Desired final state

This generated bucket is closed as superseded by issue 232's non-local module
specifier diagnostic behavior. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 232's non-local specifier diagnostic behavior
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
- [x] Superseding issue contains the exact module graph diagnostic family
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsMultiModuleMultiFile.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsMultiModuleMultiFile.ts
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

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentsMultiModuleMultiFile.ts`

## Duplicate detection

- `issues/open/232-resolve-local-relative-es-module-graph.md` covers this
  issue's current first blocker: source-spanned rejection of non-local/bare
  module specifiers with an issue-linked diagnostic.
- `issues/open/5229-resolve-imports-between-filename-sections.md` is related
  but not exact. It covers local relative imports such as `./b` between
  `@Filename` sections; this file uses a non-local specifier
  `commentsMultiModuleMultiFile_0`.
- `issues/open/436-implement-module-resolution.md` is a broad triage bucket and
  should not duplicate the completed issue-232 diagnostic boundary.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Triage backend io: commentsMultiModuleMultiFile

- Issue class: triage-needed
- Feature label: backend-io
- Diagnostic: BackendIo / backend-io
- Path: reference/typescript/tests/cases/compiler/commentsMultiModuleMultiFile.ts
```

Source context:

```ts
// @Filename: commentsMultiModuleMultiFile_0.ts
export namespace multiM {
    export class b {
    }
}

// @Filename: commentsMultiModuleMultiFile_1.ts
import m = require('commentsMultiModuleMultiFile_0');
```

Compiler evidence:

```text
tokens: ok through export namespace blocks, class exports, new multiM.* expressions, and import-equals require
ast: ok; includes New(Member(Ident("multiM"), "b")), New(Member(Ident("multiM"), "c")), ImportDefault source "commentsMultiModuleMultiFile_0", and New(Member(Ident("multiM"), "d"))
resolved: module_graph rejects non-local specifier with issue-232
wat: same module_graph issue-232 diagnostic before backend emission
```

Visible symbols include exported classes `b`, `c`, `e`, `d`, and `f` from the
virtual namespace sections.

TypeScript oracle:

```text
TS2307 Cannot find module 'commentsMultiModuleMultiFile_0' or its corresponding type declarations.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentsMultiModuleMultiFile.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedModule:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentsMultiModuleMultiFile.ts
result: issue-232 unsupported non-local module specifier, covered by done issue 232
date: 2026-05-07
```

Remaining risks:

- If future policy adds TypeScript path mapping or non-relative virtual module
  resolution, that should be tracked by a new design/implementation issue, not
  by this generated comment bucket.
