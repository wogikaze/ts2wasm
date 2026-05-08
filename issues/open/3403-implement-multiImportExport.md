---
id: 3403
title: "Split multiImportExport bucket to exported require import-equals issue"
type: maintenance
area: frontend/parser
class: superseded
priority: P1
depends_on: [432, 5430]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import-export bucket by splitting the current failure to
issue 5430. Fresh triage shows the first blocker is parsing an exported
import-equals declaration whose target is `require(...)`.

## Problem

The original bucket listed one `multiImportExport` reference file under
`import-export` without smart-triage evidence.

Fresh focused coverage reports:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

## Current failure

Smart triage reports:

```text
UnsupportedModule: issue-055: unsupported static export; module resolution and loading are not implemented at 198..204
```

Current source context:

```ts
// @Filename: Drawing.ts
export import Math = require('./Math/Math')
```

Compiler evidence:

```text
tokens: ok through exported import-equals with require target
ast/resolved: fails at the export keyword before representing ImportEqualsDeclaration
TypeScript oracle: topLevel includes ImportEqualsDeclaration for the exported require alias
```

## Desired final state

This generated bucket remains closed. The exported `import = require(...)`
parser blocker is owned by
`issues/open/5430a-parse-exported-import-equals-require-declarations.md`.

## Scope

Completed:

- [x] Re-ran focused coverage for the affected reference file.
- [x] Re-ran smart triage for the affected reference file.
- [x] Confirmed the current first blocker is exported import-equals parsing.
- [x] Created focused implementation-ready issue 5430.

Out of scope:

- Direct implementation from this generated bucket.
- CommonJS module resolution/loading.
- CommonJS `export = expr` parsing later in the same reference.

## Affected paths

Referenced only:

- `reference/typescript/tests/cases/compiler/multiImportExport.ts`

## Acceptance criteria

- [x] Current first diagnostic state is recorded.
- [x] Focused child issue 5430 owns the current blocker.
- [x] This bucket is moved to `done/`.

## Validation

Commands run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiImportExport.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiImportExport.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; no Rust source changes.

## Notes

Issue 5400 covers exported import-equals declarations with qualified-name
targets. This reference uses the external-module-reference form
`require("./Math/Math")`, so it was split into issue 5430.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: focused coverage and triage listed above
result: pass; first blocker is issue-055 at exported import-equals require form
date: 2026-05-08
```

Remaining risks:

- After issue 5430 lands, this file may expose CommonJS module resolution,
  `export =`, or duplicate export diagnostics.
