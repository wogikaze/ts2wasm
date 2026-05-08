---
id: 3371
title: "Implement Moduleresolution Module Resolution"
type: maintenance
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5007, 5402, 5422]
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

Closed as split/superseded. Fresh triage shows the seven
module-resolution paths are non-code virtual `@Filename` sections being parsed
as TypeScript source: six `package.json` cases owned by issue 5402, and one
README.md case split to new issue 5422.

## Problem

Reference test results show 7 cases fail in directory
`moduleResolution-module-resolution` with diagnostics: module-resolution. Fresh
coverage on 2026-05-08 reports six `moduleResolution_packageJson_*` cases and
`moduleResolution_noLeadingDot.ts` as `UnsupportedSyntax/module-resolution`.

Problem: this generated bucket is too broad for direct implementation. The
current blocker is not package lookup yet; the harness parses non-code virtual
sections as TypeScript source.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolution_packageJson_scopedPackage.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_packageJson_scopedPackage.ts --detail
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
- [x] Existing issue 5402 owns the package.json virtual-section parsing boundary
- [x] New issue 5422 owns the README.md virtual-section parsing boundary
- [x] Child/owner evidence includes failing path, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence, and exact diagnostic/stdout change

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_packageJson --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_noLeadingDot.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_packageJson_scopedPackage.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_noLeadingDot.ts
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Not run:

- Cargo gates; no Rust source changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created `issues/open/5422-skip-markdown-filename-sections-in-reference-harness.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolution_packageJson_scopedPackage.ts`
- `reference/typescript/tests/cases/compiler/moduleResolution_packageJson_notAtPackageRoot.ts`
- `reference/typescript/tests/cases/compiler/moduleResolution_packageJson_notAtPackageRoot_fakeScopedPackage.ts`
- `reference/typescript/tests/cases/compiler/moduleResolution_noLeadingDot.ts`
- `reference/typescript/tests/cases/compiler/moduleResolution_packageJson_yesAtPackageRoot.ts`
- `reference/typescript/tests/cases/compiler/moduleResolution_packageJson_yesAtPackageRoot_mainFieldInSubDirectory.ts`
- `reference/typescript/tests/cases/compiler/moduleResolution_packageJson_yesAtPackageRoot_fakeScopedPackage.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh runs on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_packageJson --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_noLeadingDot.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_packageJson_scopedPackage.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_noLeadingDot.ts
```

Coverage result:

```text
moduleResolution_packageJson*: executed=6, unsupported=6, UnsupportedSyntax/module-resolution
moduleResolution_noLeadingDot.ts: executed=1, unsupported=1, UnsupportedSyntax/module-resolution
```

Representative package.json evidence:

```text
path: moduleResolution_packageJson_scopedPackage.ts
tokens: ok; virtual package.json object tokens, export const x, import from "@foo/bar"
ast/resolved: UnsupportedSyntax expected Semicolon, got Some(Colon) at the package.json "types" property
TypeScript raw-source oracle: TS1005 at the JSON colon, merged declaration diagnostics, and TS2307 for @foo/bar
owner: issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md
```

README.md evidence:

```text
path: moduleResolution_noLeadingDot.ts
tokens: ok; README text tokenizes as identifiers `This is a test`, followed by `true;`
ast/resolved: UnsupportedSyntax expected Semicolon, got Some(Ident("is"))
TypeScript raw-source oracle: diagnostics for README text, then AST expression statements
owner: issues/open/5422-skip-markdown-filename-sections-in-reference-harness.md
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- local closure commit; see git log for this issue file

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_packageJson --detail --no-dashboard-data
result: pass; executed=6, unsupported=6, mapped to issue 5402
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolution_noLeadingDot.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, split to issue 5422
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_packageJson_scopedPackage.ts
result: pass; current blocker is virtual package.json property-colon parsing, owned by issue 5402
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolution_noLeadingDot.ts
result: pass; current blocker is virtual README.md text parsed as source, split to issue 5422
date: 2026-05-08
```

Remaining risks:

- After 5402 and 5422 land, these references may expose package lookup,
  scoped package resolution, declaration-file selection, or @types folder
  filtering work.


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

Resolved on 2026-05-08 by fresh triage and split. Package-json virtual section
failures are superseded by issue 5402, and the README.md virtual section
failure is split to issue 5422.

superseded-by: 5402,5422
