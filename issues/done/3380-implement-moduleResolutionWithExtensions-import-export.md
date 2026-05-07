---
id: 3380
title: "Implement Moduleresolutionwithextensions Import Export (audit reopened #3380)"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432, 5229, 5292]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage moduleResolutionWithExtensions-import-export across 5 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 5 cases in
`moduleResolutionWithExtensions-import-export`. Fresh triage splits the current
first blockers across existing owners:

- `notSupported`, `notSupported2`, and `notSupported3` parse default imports
  from empty virtual `.tsx` / `.jsx` / `.js` sections and then stop at issue-232
  missing local module diagnostics, owned by issue `5229`.
- `withAmbientPresent` parses the ambient module and named import, then stops
  at completed issue `232`'s unsupported non-local module specifier boundary
  for bare specifier `js`.
- `withPaths` stops in the virtual `/tsconfig.json` body, owned by issue
  `5292`.

Problem: this generated bucket is superseded by existing narrower owners and
should not be implemented directly.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_notSupported.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_notSupported.ts --detail
```

## Desired final state

This generated bucket is closed. The actionable blockers are tracked by issues
`5229` and `5292`, with the completed non-local specifier boundary covered by
issue `232`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede with existing implementation-ready issues `5229` and `5292`
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the superseding issue notes

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
- [x] Superseding issues contain exact `reference-triage` ownership evidence
- [x] Superseding issues include failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Superseding issues or this close evidence name the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 10
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_notSupported.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_notSupported.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; this close only moves a
  generated triage bucket and updates issue metadata, with no Rust source
  changes.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by existing issues `5229`, `5292`, and completed issue `232`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_notSupported.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_notSupported2.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_withAmbientPresent.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_notSupported3.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_withPaths.ts`

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

## Smart triage

Generated manually on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-coverage tsc \
  --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_ \
  --detail --no-dashboard-data
```

Result for the full prefix window:

```text
executed=7
build_pass=0
unsupported=7
unsupported_diagcodes=UnsupportedModule:4,UnsupportedSyntax:3
unsupported_features=import-export:4,module-resolution:3
```

The 5 files owned by this bucket map as follows:

```text
moduleResolutionWithExtensions_notSupported.ts: issue-232 missing local module `./tsx` after parsing imports; superseded by issue 5229
moduleResolutionWithExtensions_notSupported2.ts: issue-232 missing local module `./jsx` after parsing imports; superseded by issue 5229
moduleResolutionWithExtensions_notSupported3.ts: issue-232 missing local module `./jsx` after parsing imports; superseded by issue 5229
moduleResolutionWithExtensions_withAmbientPresent.ts: issue-232 unsupported non-local module specifier `js`; completed issue 232 policy boundary
moduleResolutionWithExtensions_withPaths.ts: `tsconfig.json` JSON property colon; superseded by issue 5292
```

Representative focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_notSupported.ts
```

Observed:

```text
tokens/ast: ok for import tsx/jsx/js from local specifiers
resolved/module_graph: issue-232 missing local module `./tsx`
source:
// @Filename: /tsx.tsx
// @Filename: /jsx.jsx
// @Filename: /js.js
// @Filename: /a.ts
import tsx from "./tsx";
import jsx from "./jsx";
import js from "./js";
```

Additional focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_withPaths.ts
```

Observed:

```text
diagnosis: UnsupportedSyntax / parser-or-frontend-unsupported
message: expected Semicolon, got Some(Colon) at 20..21
actual AST/resolved error: expected Semicolon, got Some(Colon) at 71..72
source:
// @filename: /tsconfig.json
{
  "compilerOptions": {
```

`moduleResolutionWithExtensions_withAmbientPresent.ts` was also triaged:

```text
tokens: ok for declare module "js" and import { x } from "js"
resolved/module_graph: issue-232 unsupported non-local module specifier `js`
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_ --detail --no-dashboard-data
result: pass; reproduced 7-prefix window and mapped the 5 issue-3380 files to existing owners
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_notSupported.ts
result: pass; first actionable blocker is virtual local import resolution, superseded by issue 5229
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_withPaths.ts
result: pass; first actionable blocker is virtual tsconfig parsing, superseded by issue 5292
date: 2026-05-08
```

Remaining risks:

- After issues `5229` and `5292`, these references may advance to extension
  filtering, JSX/allowJs policy, path mapping, or package/module resolution
  diagnostics.

## Close note

Superseded by issues `5229` and `5292`, plus completed issue `232` for the
existing non-local module specifier boundary. Fresh triage shows no unique
import/export implementation slice remains in this generated bucket.

superseded-by: 5229, 5292, 232

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/3380-implement-moduleResolutionWithExtensions-import-export.md` before this move
- `issues/done/3380-implement-moduleResolutionWithExtensions-import-export.md` after this move

Split follow-up: none created; existing issues `5229` and `5292` are the
tracking items for the current open blockers.
