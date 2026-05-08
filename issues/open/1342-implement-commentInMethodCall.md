---
id: 1342
title: "Implement Commentinmethodcall"
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

Closed this generated bucket by splitting the current concrete blocker to
`issues/open/5343-track-array-typed-erased-locals-for-callback-methods.md`.

## Problem

Fresh triage shows comments are not the current blocker. The lexer skips the
file comments and inline call-argument comment, the parser builds the method
call AST, and lowering then reports `issue-211: unknown receiver class for
method map` for `s.map(...)` because `var s: string[];` is erased to an
unknown declaration-only local.

Problem: `commentInMethodCall.ts` is blocked by array-typed erased local
receiver tracking, now owned by issue 5343.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentInMethodCall.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentInMethodCall.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implement from
`issues/open/5343-track-array-typed-erased-locals-for-callback-methods.md`.

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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentInMethodCall.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentInMethodCall.ts
```

Not run:

- cargo fmt / nextest not run for this metadata-only issue lifecycle split

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5343-track-array-typed-erased-locals-for-callback-methods.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/commentInMethodCall.ts`

## Duplicate detection

No exact existing owner was found for declaration-only `var s: string[];`
followed by `s.map(...)`.

No-match candidates:

- `issues/open/5234-w0-implement-host-deny-and-auditable-e2e-manifest-verification.md`
  covers array-shaped function/class-method parameters, not declaration-only
  locals.
- `issues/open/5222a-parse-ambient-generic-variable-type-annotations.md`
  covers interface-typed erased locals such as `Sequence<string>`, not array
  callback receivers.
- `issues/done/297-track-pushed-dense-array-locals-for-map.md` covers
  initialized dense arrays built through pushes, not erased declaration-only
  locals.
- `issues/open/435-implement-method-call.md` is a broad umbrella and does not
  carry this reference path's array-typed local evidence.

## Smart triage

Generated 2026-05-07 with:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentInMethodCall.ts
```

Source context:

```ts
// @target: es2015
//commment here
var s: string[];
s.map(// do something
    function () { });
```

Result:

```text
Smart triage: Triage class: commentInMethodCall
Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
Message: issue-211: unknown receiver class for method map at 55..98
tokens: ok; comments are skipped and the call tokens are preserved
ast: ok; Let s = Undefined, Expr Call(Member(Ident("s"), "map"), args=[FunctionExpr {}])
resolved/lowered: fail with issue-211 unknown receiver class for method map
visible symbol: binding s at line 3, column 1
TypeScript oracle: TS2454 Variable 's' is used before being assigned; binding s has typeText string[]
```

Focused coverage:

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

## Completion evidence

Commits:

- this commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/commentInMethodCall.ts
result: pass; reproduced issue-211 unknown receiver class for method map after successful tokens/AST and split to issue 5343
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/commentInMethodCall.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax=1
date: 2026-05-07
```

Remaining risks:

- Issue 5343 still needs implementation; this closure only removes the stale generated bucket from the blocked queue.
