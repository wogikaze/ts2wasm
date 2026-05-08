---
id: 3576
title: "Implement Nodecolonmoduleresolution"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nodeColonModuleResolution across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows the two affected paths are already covered by narrower
module-resolution owners:

- `nodeColonModuleResolution.ts` reaches completed issue `232`'s deliberate
  non-local module specifier boundary for `node:ph`.
- `nodeColonModuleResolution2.ts` stops in a virtual `/a/b/tsconfig.json`
  section and is folded into issue `5292`.

Problem: the generated bucket no longer identifies an independent
implementation slice.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nodeColonModuleResolution.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nodeColonModuleResolution.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by existing issue evidence. Do
not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fold the config-section blocker into issue `5292`
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
- [x] `nodeColonModuleResolution.ts` is covered by completed issue `232`
- [x] `nodeColonModuleResolution2.ts` is covered by open issue `5292`
- [x] Closed bucket includes exact commands, diagnostic codes, source context, and TypeScript AST/oracle evidence

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeColonModuleResolution.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeColonModuleResolution.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeColonModuleResolution2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeColonModuleResolution2.ts
```

Not run:

- `cargo fmt --all --check`; metadata-only issue triage close.
- `cargo nextest run`; metadata-only issue triage close.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nodeColonModuleResolution.ts`
- `reference/typescript/tests/cases/compiler/nodeColonModuleResolution2.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

### `nodeColonModuleResolution.ts`

Fresh focused coverage:

```text
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
per-file: reference/typescript/tests/cases/compiler/nodeColonModuleResolution.ts => UnsupportedModule / import-export
```

Fresh triage reaches a parsed namespace import and then the existing
non-local specifier policy boundary:

```text
ImportNamespace local `ph`, source "node:ph"
module_graph: [UnsupportedModule] issue-232: unsupported non-local module specifier node:ph; package resolution, import maps, and absolute specifiers are not implemented
```

TypeScript oracle for the reference window also reports unresolved module
diagnostics (`TS2664` and `TS2307`) for `ph` / `node:ph`. This generated bucket
does not create a new implementation issue because completed issue `232`
explicitly owns the current unsupported non-local module specifier diagnostic.

### `nodeColonModuleResolution2.ts`

Fresh focused coverage:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-resolution:1
per-file: reference/typescript/tests/cases/compiler/nodeColonModuleResolution2.ts => UnsupportedSyntax / module-resolution
```

Fresh triage stops while parsing the virtual `/a/b/tsconfig.json` section:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Colon) at 133..134
tokens: LeftBrace, String("compilerOptions"), Colon, LeftBrace, String("paths"), Colon, ...
TypeScript AST topLevel: Block for the JSON config, ModuleDeclaration, ImportDeclaration, ExpressionStatement
```

The later source imports `fake:thing`, but path mapping behavior is not
actionable until issue `5292` skips `tsconfig.json` filename sections instead
of parsing config JSON as executable source.

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeColonModuleResolution.ts --detail --no-dashboard-data
result: pass; UnsupportedModule/import-export, current blocker issue-232 unsupported non-local module specifier node:ph
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeColonModuleResolution.ts
result: pass; parsed ImportNamespace from node:ph, resolved dump reports issue-232 non-local module boundary
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeColonModuleResolution2.ts --detail --no-dashboard-data
result: pass; UnsupportedSyntax/module-resolution in virtual tsconfig.json section
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeColonModuleResolution2.ts
result: pass; expected Semicolon got Colon at the tsconfig.json compilerOptions property
date: 2026-05-08
```

Remaining risks:

- After issue `5292` skips the `tsconfig.json` section,
  `nodeColonModuleResolution2.ts` may expose a later `fake:thing` path-mapping
  or non-local specifier blocker.
