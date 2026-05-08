---
id: 3385
title: "Implement Moduleresolutionwithsuffixes Import Export (audit reopened #3385)"
type: maintenance
area: compiler/multi-section
class: superseded
priority: P1
depends_on: [432, 5292]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage moduleResolutionWithSuffixes-import-export across 12 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh prefix coverage for `moduleResolutionWithSuffixes_` now reports 16
unsupported paths. Representative triage shows the current first blocker is
not import/export or suffix resolution yet: each sampled path starts with a
virtual `/tsconfig.json` body, and the multi-section compiler path parses that
JSON as TypeScript source.

Problem: the suffixes import/export bucket is currently blocked by
`tsconfig.json` virtual-section parsing, already owned by issue 5292.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_notSpecified.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_ --detail --no-dashboard-data
```

Observed result:

```text
executed=16
unsupported=16
unsupported_diagcodes=UnsupportedSyntax:16
unsupported_features=module-resolution:16
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`.
After issue 5292 lands, these references may expose actual moduleSuffixes,
JSON module, JS module, external package, or virtual-section import blockers.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with an existing implementation-ready owner
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing issue 5292 contains the implementation-ready tsconfig harness task
- [x] This close records failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Follow-up issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

Issue-only close; Rust gates were not required for this lifecycle split.

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_ --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_notSpecified.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_one_jsonModule.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_one_jsModule.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_one_externalModule.ts
```

Not run:

- cargo fmt --all --check
- cargo nextest run

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_notSpecified.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_empty.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_oneBlank.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_oneNotFound.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_one.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_one_dirModuleWithIndex.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_one_jsModule.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_one_externalTSModule.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_threeLastIsBlank2.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_threeLastIsBlank1.ts`
- ... and 2 more files

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/open/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/done/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)
- Fresh triage confirms these import/export duplicates do not own the current
  first blocker. The current failure is virtual `tsconfig.json` parsing,
  covered by issue 5292.

## Smart triage

Fresh coverage:

```text
executed=16
unsupported=16
unsupported_diagcodes=UnsupportedSyntax:16
unsupported_features=module-resolution:16
```

Representative `moduleResolutionWithSuffixes_notSpecified.ts`:

```text
first_code_line: {
failure: UnsupportedSyntax expected Semicolon, got Some(Colon) at the `compilerOptions` property
visible symbols: []
tokens: tsconfig object tokens, then `import { base } from "./foo";`, then `export function base() {}`
TypeScript AST top-level: Block for tsconfig body, ImportDeclaration, FunctionDeclaration
```

Representative `moduleResolutionWithSuffixes_one_jsonModule.ts`:

```text
failure: same tsconfig `compilerOptions` property-colon boundary
later source contains `import foo from "./foo.json";` and virtual `.json` bodies, but those are not reachable before issue 5292
```

Representative JS/external-module paths show the same first failure in
`/tsconfig.json` before later JS `exports.*`, package, JSON module, or
moduleSuffixes behavior becomes actionable.

## Completion evidence

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_ --detail --no-dashboard-data
result: pass; 16 UnsupportedSyntax/module-resolution paths, all sampled first blockers are tsconfig JSON parsing
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_notSpecified.ts
result: pass; stops in virtual `/tsconfig.json` at `compilerOptions` property colon, superseded by issue 5292
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_one_jsonModule.ts
result: pass; stops in virtual `/tsconfig.json` before JSON module import or JSON fixture bodies become reachable
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_one_jsModule.ts
result: pass; stops in virtual `/tsconfig.json` before JS module sections become reachable
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithSuffixes_one_externalModule.ts
result: pass; stops in virtual `/tsconfig.json` before external package/moduleSuffixes behavior becomes reachable
date: 2026-05-08
```

Remaining risks:

- After issue 5292 lands, the 16 suffixes paths may split across actual
  moduleSuffixes resolution, JSON module, JS module, package, and virtual
  import blockers.
## Close note

Superseded by issue 5292, which owns virtual `tsconfig.json` sections being
parsed as executable source in the reference multi-section harness.

superseded-by: 5292

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/3385-implement-moduleResolutionWithSuffixes-import-export.md` before this move
- `issues/done/3385-implement-moduleResolutionWithSuffixes-import-export.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
