---
id: 1385
title: "Implement Commonjsisolatedmodules"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: [5292]
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Closed after splitting the current reference-harness blocker to
`issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`.

Fresh focused triage shows the failing boundary is not isolatedModules
semantics itself. The compiler is parsing the virtual `tsconfig.json` section
as executable TypeScript/JavaScript source before reaching `index.js`.

## Problem

Reference test results originally showed 1 case failing in directory
`commonJsIsolatedModules` with diagnostics: import-export. Fresh focused
coverage on 2026-05-07 reports `UnsupportedSyntax` for the same path.

Problem: `commonJsIsolatedModules.ts` contains `// @Filename: tsconfig.json`
followed by `// @Filename: index.js`. The multi-section path treats the JSON
config body as a module body, so AST/resolved dumps fail after the JSON
`RightBrace` while parsing resumes at the `index.js` `module` token.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
coverage: build_pass=0, unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
ast/resolved: unsupported expression ... RightBrace ... at 173..179
```

## Desired final state

This generated bucket is closed. Implementation should proceed through the
focused child issue that handles `tsconfig.json` virtual sections in the
reference multi-section harness.

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
- [x] Child issue 5292 contains an exact `python scripts/manager.py reference-triage ...` command
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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts
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

- [x] added: `issues/open/5292-skip-tsconfig-filename-sections-in-reference-harness.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts`

## Duplicate detection

- No exact existing implementation-ready issue was found for
  `tsconfig.json` virtual section handling.
- `issues/done/5229-w0-user-runtime-string-origin.md` is related
  but owns local import resolution between code sections, not config-section
  filtering.
- `issues/done/5187-lower-namespace-only-multi-section-files.md` is related
  but owns namespace-only code sections, not JSON config sections.

## Smart triage

Generated 2026-05-07.

```text
### Smart triage: Build pass: commonJsIsolatedModules

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts
```

Source context:

```text
// @target: es2015
// @Filename: tsconfig.json
{
  "compilerOptions": {
    "allowJs": true,
    "outDir": "foo",
    "isolatedModules": true,
  }
}

// @Filename: index.js
module.exports = {}
var x = 1
```

Compiler evidence:

```text
tokens: ok; JSON object tokens and index.js tokens are present
ast: UnsupportedSyntax unsupported expression near RightBrace at 173..179
resolved: same UnsupportedSyntax
visible symbols: binding x at line 13
```

TypeScript oracle:

```text
The raw unsplit .ts file reports syntax/name diagnostics, including TS2591 for
module. This is not the desired harness behavior; the actionable blocker is the
reference multi-section compiler path treating tsconfig.json as source.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this commit

Validation result:

```text
command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts --detail --no-dashboard-data
result: build_pass=0, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commonJsIsolatedModules.ts
result: AST/resolved fail after tsconfig.json section; split to issue 5292
date: 2026-05-07
```

Remaining risks:

- After issue 5292 skips or consumes the config section, the same reference
  path may expose a CommonJS `module.exports` lowering blocker.
