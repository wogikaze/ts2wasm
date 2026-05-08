---
id: 3567
title: "Implement Nosymbolformergecrash"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5187]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage noSymbolForMergeCrash across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh coverage now reports a build pass, but direct smart triage still stops at
the unspanned multi-section empty-body guard. This is the same
`multi-section file has no module bodies` boundary already owned by issue 5187.

Problem: `noSymbolForMergeCrash.ts` is superseded by issue 5187 until
namespace/type-only `// @Filename:` sections remain observable enough for the
later TS2300 duplicate identifier diagnostics to be inspected.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noSymbolForMergeCrash.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noSymbolForMergeCrash.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with issue 5187
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
- [x] Issue 5187 contains the exact multi-section empty-body boundary
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture and diagnostic/stdout change required after issue 5187

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/noSymbolForMergeCrash.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/noSymbolForMergeCrash.ts
```

Not run:

- broad Rust gates; no source implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5187-lower-namespace-only-multi-section-files.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noSymbolForMergeCrash.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh triage on 2026-05-08:

- Coverage: `executed=1`, `build_pass=1`, `unsupported=0`, `blocked=0`
- Direct triage diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Direct triage message: `multi-section file has no module bodies`
- Source context: `// @Filename: initial.ts` with `interface A { }` and
  `namespace A {}`, followed by `// @Filename: final.ts` with `type A = {}`.
- Tokens are ok for the interface, namespace, and type alias declarations.
- AST/resolved dumps are empty, matching issue 5187's namespace/type-only
  multi-section empty-body boundary.
- TypeScript oracle reports TS2300 duplicate identifier diagnostics for `A`,
  but those are hidden until issue 5187 preserves a non-empty section or emits
  a more precise section diagnostic.

## Completion evidence

Superseded by issue 5187.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noSymbolForMergeCrash.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noSymbolForMergeCrash.ts
result: pass; direct triage still reports `multi-section file has no module bodies`, superseded by issue 5187
date: 2026-05-08
```

Remaining risks:

- After issue 5187 advances this path past the empty-body guard, TS2300
  duplicate identifier parity may need separate follow-up work.
