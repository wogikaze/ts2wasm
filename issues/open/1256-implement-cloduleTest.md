---
id: 1256
title: "Implement Cloduletest"
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
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1256.

## Summary

Closed as superseded by
`issues/open/5217-preserve-ambient-value-declarations-through-name-resolution.md`.

## Problem

Reference test results originally showed 2 cases failing in directory
`cloduleTest` with diagnostics: import-export. Fresh focused coverage on
2026-05-07 now shows `cloduleTest2.ts` build-passes and `cloduleTest1.ts`
stops at issue-211 method-call receiver lowering.

Problem: `cloduleTest1.ts` parses `$('.foo').addClass('bar')`, but lowering
rejects the `addClass` method call because the receiver is a call expression
instead of an identifier. This exact receiver shape is already owned by issue
5217.

## Current failure

Current representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleTest1.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleTest --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5217,
which owns method calls on call-expression receivers.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with the existing call-expression receiver issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad method-call support beyond issue 5217
- TypeScript semantic diagnostic parity for `cloduleTest2.ts`

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
- [x] Existing issue 5217 covers the current `$('.foo').addClass('bar')` blocker
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference paths and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleTest --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleTest1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleTest2.ts
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

- [x] superseded by: `issues/open/5217-preserve-ambient-value-declarations-through-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cloduleTest2.ts`
- `reference/typescript/tests/cases/compiler/cloduleTest1.ts`

## Duplicate detection

- `issues/open/5217-preserve-ambient-value-declarations-through-name-resolution.md`
  owns method calls where the receiver is itself a call expression, matching
  `$('.foo').addClass('bar')`.
- `issues/open/5221-support-bitwise-and-xor-binary-lowering.md`
  is related but covers longer chained `.then` expressions.
- `issues/open/5142-support-class-method-call-on-new-expression-receiver.md`
  is related but only covers `new C().g()`.

## Smart triage

Generated on 2026-05-07.

Focused coverage:

```text
executed=2
build_pass=1
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
reference/typescript/tests/cases/compiler/cloduleTest1.ts: UnsupportedSyntax: unknown-unsupported
reference/typescript/tests/cases/compiler/cloduleTest2.ts: build_pass
```

### cloduleTest1

```text
### Smart triage: Triage method call: cloduleTest1

- Issue class: triage-needed
- Feature label: method-call
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/cloduleTest1.ts
```

Current diagnostic:

```text
issue-211: method `addClass` requires an identifier receiver at 260..285
```

Source context:

```ts
declare function $(selector: string): $;
interface $ {
    addClass(className: string): $;
}
namespace $ {
    export interface AjaxSettings {
    }
    export function ajax(options: AjaxSettings) { }
}
var it: $ = $('.foo').addClass('bar');
```

Compiler evidence:

```text
tokens: ok through declare function $, interface $, namespace $, and addClass call
ast: Let it = Call(Member(Call(Ident("$"), ".foo"), property="addClass"), "bar")
resolved/lowered: issue-211 because addClass has a call-expression receiver
visible symbols: function $, function ajax, binding it
```

TypeScript oracle evidence:

```text
typescriptVersion: 6.0.3
diagnostics: []
binding hint: it has type $
```

This is the same implementation slice as issue 5217:
`Support method calls on call expression receivers`.

### cloduleTest2

Fresh triage reports `BuildPass`. TypeScript oracle still reports semantic
diagnostics such as TS2554 constructor arity and TS2339/TS2576 instance-vs-static
member access, but current compiler build coverage no longer has a blocker for
this file.

## Completion evidence

Closed as superseded on 2026-05-07.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleTest1.ts
result: pass; reproduced issue-211 call-expression receiver blocker superseded by issue 5217
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleTest2.ts
result: pass; current compiler build-passes
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleTest --detail --no-dashboard-data
result: pass; executed=2, build_pass=1, unsupported=1
date: 2026-05-07
```

Remaining risks:

- TypeScript semantic diagnostic parity for `cloduleTest2.ts` remains outside
  this generated build-blocker cleanup.
