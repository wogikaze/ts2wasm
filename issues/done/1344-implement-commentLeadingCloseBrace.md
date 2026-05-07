---
id: 1344
title: "Implement Commentleadingclosebrace"
type: spike
area: frontend/resolver
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

Fresh triage for `commentLeadingCloseBrace.ts` returns `BuildPass`. The
compiler tokenizes and parses the `declare function`, `if/else` body, block
comments, closing braces, and all `commentedParameters(...)` calls; name
resolution also succeeds.

Problem: no current compiler blocker remains for this generated comment/close
brace bucket.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentLeadingCloseBrace.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentLeadingCloseBrace.ts --detail --no-dashboard-data
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
- [x] Closure includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentLeadingCloseBrace.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentLeadingCloseBrace.ts
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

- `reference/typescript/tests/cases/compiler/commentLeadingCloseBrace.ts`

## Duplicate detection

No duplicate implementation issue is needed because the current focused window
is build-pass. The only same-path duplicate candidate from fresh triage was
this generated bucket itself.

## Smart triage

Generated 2026-05-07 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentLeadingCloseBrace.ts
```

Result:

```text
Smart triage: Build pass: commentLeadingCloseBrace
Diagnostic: BuildPass / pass
Feature label: build-pass
tokens: ok through declare function, if/else, comments, and closing braces
ast: ok; Function commentedParameters plus Function ifelse with If/then/else call bodies
resolved: ok; all commentedParameters calls resolve
visible symbols: commentedParameters at line 3, ifelse at line 5
TypeScript oracle: ok, diagnostics=[], hints commentedParameters:any, args:any[], ifelse:void
```

Source context:

```ts
declare function commentedParameters(...args): any;

function ifelse() {
    if (commentedParameters(1, 2)) {
        /*comment1*/
        commentedParameters(3, 4);
        /*comment2*/
    } else {
        commentedParameters(5, 6);
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
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentLeadingCloseBrace.ts
result: pass; BuildPass with TypeScript oracle diagnostics=[]
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentLeadingCloseBrace.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0, blocked=0
date: 2026-05-07
```

Remaining risks:

- Semantic runtime parity is not enabled in this focused coverage path, but no current compiler build blocker remains for this generated bucket.
