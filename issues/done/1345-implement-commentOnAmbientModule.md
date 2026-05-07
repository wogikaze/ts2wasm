---
id: 1345
title: "Implement Commentonambientmodule"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed this generated bucket as stale after fresh triage and focused coverage
showed the reference file now builds successfully.

## Problem

Fresh triage for `commentOnAmbientModule.ts` returns `BuildPass`. The file is
a multi-section TypeScript reference test with `declare namespace C`, `D`, and
`E`; the compiler tokenizes those ambient namespaces and their class/function
members, erases the bodies to an empty AST, and finishes the build. TypeScript
oracle also reports no diagnostics.

Problem: no current compiler blocker remains for this generated ambient
module/comment bucket.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientModule.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnAmbientModule.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. No child issue was created because the current
focused reference window has no build blocker.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closure

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is stale
- [x] Closure contains an exact `reference-triage` command
- [x] Closure includes path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] No child acceptance needed because the focused reference window is build-pass

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnAmbientModule.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientModule.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle closure

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentOnAmbientModule.ts`

## Duplicate detection

No duplicate implementation issue is needed because the current focused window
is build-pass. Adjacent ambient/namespace issues such as
`issues/open/5187-lower-namespace-only-multi-section-files.md` and
`issues/open/5172-report-unresolved-implements-in-erased-namespace.md` own
other reference files with real diagnostics; this path has no current compiler
or TypeScript oracle failure.

## Smart triage

Generated 2026-05-07 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientModule.ts
```

Result:

```text
Smart triage: Build pass: commentOnAmbientModule
Diagnostic: BuildPass / pass
Feature label: build-pass
tokens: ok through declare namespace C/D/E, function foo, class bar, class foobar extends D.bar
ast: ok; []
resolved: ok; []
visible symbols: foo at line 11, bar at line 16, foobar at line 22
TypeScript oracle: ok, diagnostics=[], hint foo:any
```

Source context:

```ts
//@filename: a.ts
/*!=========
    Keep this pinned comment
   =========
*/

/*! Don't keep this pinned comment */
declare namespace C {
    function foo();
}

// Don't keep this comment.
declare namespace D {
    class bar { }
}

//@filename: b.ts
///<reference path="a.ts"/>
declare namespace E {
    class foobar extends D.bar {
        foo();
    }
}
```

Focused coverage:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
unsupported_diagcodes=
unsupported_features=
semantic_enabled=0
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentOnAmbientModule.ts
result: pass; BuildPass with TypeScript oracle diagnostics=[]
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentOnAmbientModule.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-07
```

Remaining risks:

- Semantic runtime parity is not enabled in this focused coverage path, but no current compiler build blocker remains for this generated bucket.
