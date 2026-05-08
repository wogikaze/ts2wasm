---
id: 3336
title: "Implement Moduleexportnonstructured"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated module bucket as superseded by the existing
`package.json` virtual-section handling issue.

## Problem

Fresh triage shows the current first blocker is not module export syntax. The
reference file starts with a virtual `package.json` section, and the compiler
parses that JSON object as TypeScript source:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 133..134
```

The later `.mts` and `.d.cts` import/export sections are not reachable until the
reference harness skips or records the package metadata section instead of
parsing it as executable code.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleExportNonStructured --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExportNonStructured.ts
```

First failing section:

```ts
// @filename: package.json
{
    "name": "test",
    "version": "1.0.0",
    "description": "",
    "type": "module",
    "module": "index.mjs"
}
```

Compiler evidence:

```text
tokens: ok; LeftBrace, String("name"), Colon, String("test"), ...
ast: UnsupportedSyntax expected Semicolon, got Some(Colon) at 133..134
resolved: same parser failure
```

TypeScript oracle evidence:

```text
TS1005: ';' expected at the JSON property colon
TS2307: Cannot find module './exportAny.cjs'
TS2307: Cannot find module './exportUnknown.cjs'
TS2307: Cannot find module './exportSymbol.cjs'
TS2451: Cannot redeclare block-scoped variable '__'
TS2300: Duplicate identifier 'export='
```

The oracle confirms several later diagnostics, but the actionable ts2wasm first
blocker is the `package.json` section boundary already owned by issue 5402.

## Desired final state

Implement the first blocker in
`issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`.
After that lands, rerun this case to expose the next import/export, module
resolution, or declaration-file diagnostic.

## Scope

In scope:

- [x] Confirm fresh smart-triage evidence for this generated bucket.
- [x] Match the current first blocker to an existing implementation-ready issue.
- [x] Preserve later import/export and declaration-file diagnostics as risk.

Out of scope:

- Direct implementation from this generated bucket.
- Full package.json semantics.
- `.cjs`/`.d.cts` module resolution or `export =` behavior after metadata
  section handling advances.

## Affected paths

Expected implementation owner:

- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch from this bucket:

- backend/runtime emit
- broad package resolution implementation

## Acceptance criteria

- [x] Superseding issue identified: `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`.
- [x] Exact reproduction commands and observed diagnostics are recorded.
- [x] Later `.cjs` and `export =` diagnostics are recorded as follow-up risk.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
git diff --cached --check
```

Reference commands already run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleExportNonStructured --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExportNonStructured.ts
```

Not run:

- `cargo fmt --all --check` (issue lifecycle only; no Rust changes)
- `cargo nextest run` (issue lifecycle only; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] existing issue 5402 owns the first blocker

## Notes

Superseded by
`issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`.

## Completion evidence

Commits:

- filled by commit

Validation result:

```text
command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-08

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-08

command: git diff --check
result: pass
date: 2026-05-08

command: git diff --cached --check
result: pass
date: 2026-05-08
```

Remaining risks:

- Once issue 5402 skips `package.json` sections, this case may stop on `.cjs`
  resolution, `import type * as`, `.d.cts`, or CommonJS `export =` diagnostics.
