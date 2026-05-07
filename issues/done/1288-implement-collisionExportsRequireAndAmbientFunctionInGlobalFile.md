---
id: 1288
title: "Implement Collisionexportsrequireandambientfunctioninglobalfile"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed as stale. Fresh triage and focused coverage show
`reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts`
now build-passes, so there is no current import/export compiler blocker to
split into a child issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionExportsRequireAndAmbientFunctionInGlobalFile` with diagnostics:
import-export. Fresh triage shows the current compiler builds the
representative path.

Problem: the generated import/export bucket is stale. TypeScript oracle also
reports no diagnostics for this path in the current runner view, so there is no
diagnostic-parity child to split from this bucket.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as stale because the representative path now
reports `build_pass`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the representative case is now build-pass with no active import/export blocker
- [x] Close as stale build-pass instead of creating a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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

- [x] Duplicate candidates below are confirmed; no separate issue is needed
- [x] No child issue needed because the representative case now build-passes
- [x] This issue includes path, diagnostic status, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and build-pass result

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current compiler build has no import/export blocker on this path

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts`

## Duplicate detection

No exact implementation child is created because the original import/export
blocker no longer reproduces. Fresh triage only found this generated bucket as
a same-path duplicate candidate.

Resolution:

```text
The original import/export blocker is stale. The reference window now reports build_pass with semantic checking disabled, so no implementation-ready blocker is split from this generated bucket.
```

## Smart triage

### Smart triage: Build pass: collisionExportsRequireAndAmbientFunctionInGlobalFile

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=1
semantic_pass=0
unsupported=0
blocked=0
unsupported_features=
semantic_enabled=0
```

Source context:

```ts
declare function exports(): number;
declare function require(): string;
declare namespace m3 {
    function exports(): string[];
    function require(): number[];
}
namespace m4 {
    export declare function exports(): string;
    export declare function require(): string;
    var a = 10;
}
```

Compiler evidence:

```text
tokens: ok; includes declare function exports/require, declare namespace m3, namespace m4, and binding a
ast: ok; Function exports and Function require are parsed as empty-body declarations
resolved: ok; Function exports and Function require
visible symbols: function exports/require in global and namespace scopes plus binding a initialized to 10
```

TypeScript oracle evidence:

```text
ok: true
diagnostics: []
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; build succeeded and original import/export blocker is stale
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionExportsRequireAndAmbientFunctionInGlobalFile.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; executed=1, build_pass=1, unsupported=0
date: 2026-05-07
```

Remaining risks:

- Semantic runtime parity is not enabled for this reference window, but there is
  no current import/export blocker or TypeScript oracle diagnostic to split
  from this generated bucket.
