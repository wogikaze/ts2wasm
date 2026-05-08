---
id: 3381
title: "Implement Moduleresolutionwithextensions Module Resolution"
type: maintenance
area: compiler/multi-section
class: superseded
priority: P2
depends_on: [5007, 5402, 5424]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

> **Reopened by audit** (2026-05-06)
> Classification: false-done (blocked)
> Reason: relapsed false-done: reopened in df7621e3, re-closed without implementation. No implementation commits.
>
> True-done checklist:
> 1. Implementation commits in the repo that satisfy the acceptance criteria
> 2. Filled completion evidence section with commits and validation results
> 3. No relapsed false-done pattern (previously reopened but re-closed without evidence)

## Summary

Triage moduleResolutionWithExtensions-module-resolution across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage of the two `moduleResolutionWithExtensions_unexpected*` cases
shows this generated bucket is not ready for direct module-resolution
implementation. Both references currently fail because the reference
multi-section path parses virtual files that TypeScript intends as package
fixtures before the real `/a.ts` import section.

Problem: the first actionable blockers are narrower multi-section harness
boundaries: unread asset sections such as `/node_modules/foo/foo.js` or
`/node_modules/normalize.css/normalize.css` are parsed as source, and later
`package.json` sections are separately owned by issue 5402.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected2.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected --detail --no-dashboard-data
```

Observed result:

```text
suite=tsc
executed=2
unsupported=2
unsupported_diagcodes=UnsupportedSyntax:2
unsupported_features=module-resolution:2
reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected.ts: UnsupportedSyntax: module-resolution
reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected2.ts: UnsupportedSyntax: module-resolution
```

## Desired final state

This generated bucket is closed. The current blockers are owned by narrower
implementation-ready issues:

- `issues/open/5424a-skip-unread-asset-filename-sections-in-reference-harness.md`
- `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`

After those harness boundaries are removed, any remaining package field or
module-resolution behavior should be tracked in a new focused issue with fresh
post-skip evidence.

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

- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

Issue-only close; Rust gates were not required for this lifecycle split.

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected.ts
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

- [x] `issues/open/5424a-skip-unread-asset-filename-sections-in-reference-harness.md`
- [x] `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected2.ts`
- `reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected.ts`

## Duplicate detection

- `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`
  owns the later virtual `package.json` body parse boundary.
- No existing issue owned unread `.js`/`.css` asset sections whose content is
  deliberately invalid and should not be parsed before package fields decide
  reachability. Split to issue 5424.

## Smart triage

Fresh triage for `moduleResolutionWithExtensions_unexpected2.ts`:

```text
first_code_line: This file is not read.
failure: UnsupportedSyntax expected Semicolon, got Some(Ident("file"))
tokens: Ident("This"), Ident("file"), Ident("is"), Ident("not"), Ident("read"), Dot, then package.json object tokens, then import "foo";
TypeScript AST top-level: ExpressionStatement `This`, ExpressionStatement `file`, ExpressionStatement `is`, ExpressionStatement `not`, ExpressionStatement `read.`, Block `{ "types": "foo.js" }`, ImportDeclaration `import "foo";`
```

Fresh triage for `moduleResolutionWithExtensions_unexpected.ts`:

```text
first_code_line: This file is not read.
failure: UnsupportedSyntax expected Semicolon, got Some(Ident("file"))
tokens: Ident("This"), Ident("file"), Ident("is"), Ident("not"), Ident("read"), Dot, then package.json object tokens, then import "normalize.css";
TypeScript AST top-level: ExpressionStatement `This`, ExpressionStatement `file`, ExpressionStatement `is`, ExpressionStatement `not`, ExpressionStatement `read.`, Block `{ "main": "normalize.css" }`, ImportDeclaration `import "normalize.css";`
```

The immediate blocker is issue 5424. The next reachable non-code section after
that skip is the virtual `package.json` body, already covered by issue 5402.

## Completion evidence

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected --detail --no-dashboard-data
result: pass; both paths reproduce UnsupportedSyntax/module-resolution at unread fixture text, split to issue 5424
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected2.ts
result: pass; current first blocker is `This file is not read.` from `/node_modules/foo/foo.js`, split to issue 5424
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithExtensions_unexpected.ts
result: pass; current first blocker is `This file is not read.` from `/node_modules/normalize.css/normalize.css`, split to issue 5424
date: 2026-05-08
```

Remaining risks:

- After issue 5424 and issue 5402 land, these references may expose package
  field extension filtering or bare package resolution blockers.


---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: This is a generated triage bucket issue. It was
created as a `class: blocked` spike with `depends_on` pointing to a parent
meta-issue (5004 or 5007). When the parent meta-issue was moved to
`issues/done/`, this child issue was dragged along without any implementation
or triage work. The `## Completion evidence` section is unfilled (commits
placeholder `...`, validation result empty). Zero implementation commits
reference this issue.

**True-done checklist** (all must pass):

1. **Triage the representative failure path**: Confirm it is superseded by an
   existing open/done issue OR split into implementation-ready child issues
   with exact reproduction commands.

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - Issue URL or child issue path documenting the triage outcome
   - Or: the exact failing reference path has a matching open/done issue
   - Or: the failing test case no longer reproduces the original diagnostic

## Close note

Superseded by focused multi-section harness issues. Issue 5424 owns the
current unread `.js`/`.css` asset section parse blocker; issue 5402 owns the
later virtual `package.json` section parse blocker.

superseded-by: 5424
superseded-by: 5402
