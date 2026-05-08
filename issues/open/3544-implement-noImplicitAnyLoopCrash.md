---
id: 3544
title: "Implement Noimplicitanyloopcrash"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed after refreshed evidence split the current lowering boundary to
`issues/open/5476-implement-number-bitwise-not-lowering.md`.

## Problem

Fresh triage shows this fixture now tokenizes, parses, resolves names, resolves
builtins, and builds typed IR. The current failure is in lowering:

```text
UnsupportedSyntax: unary operator BitwiseNot not yet supported
```

Problem: `noImplicitAnyLoopCrash.ts` is no longer a broad generated bucket; its
first current blocker is the narrow number bitwise-not lowering gap split to
issue 5476.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=unknown-unsupported:1
triage: UnsupportedSyntax unary operator BitwiseNot not yet supported
```

## Desired final state

This generated bucket is closed after splitting the current observable behavior
into an implementation-ready child issue. Do not implement directly from this
bucket.

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
- [x] Child issue 5476 contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only.
- `cargo nextest run`; issue metadata only.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to `issues/open/5476-implement-number-bitwise-not-lowering.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts`

## Duplicate detection

- `issues/open/5143-implement-unary-void-operator-lowering.md` is related
  unary lowering work but only owns `void`; `UnaryOp::Void` is already accepted
  by `lower_unary_op` in this worktree.
- BigInt bitwise-not issues 377 and 387 are done and cover BigInt-specific
  semantics, not ordinary Number `~`.
- Spread issues 274 and 353 may matter after bitwise-not lowering advances,
  but the current compiler fails before spread behavior is observable.
- Split to issue 5476.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage operator: noImplicitAnyLoopCrash

- Issue class: triage-needed
- Feature label: operator
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts
```

Current compiler message:

```text
unary operator BitwiseNot not yet supported
```

Source context:

```ts
let foo = () => {};
let bar;
while (1) {
    bar = ~foo(...bar);
}
```

Visible symbols before failure:

```text
binding: foo at line 3, initializer () => {}
binding: bar at line 4
```

Compiler evidence:

```text
tokens: ok; includes Tilde at 104..105 and DotDotDot at 109..112
ast: ok; While body assigns bar = Unary(BitwiseNot, Call(foo, [Spread(bar)]))
resolved: ok through validate_ast, module_graph, resolve_names, resolve_builtins, build_typed_ir
lowered: fails in lower_program with UnsupportedSyntax for BitwiseNot
```

TypeScript oracle:

```text
TS2556 at line 6 column 16: A spread argument must either have a tuple type or be passed to a rest parameter.
foo: () => void
bar: any
```

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts --detail --no-dashboard-data
result: pass; reproduced current unsupported boundary with UnsupportedSyntax for BitwiseNot
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyLoopCrash.ts
result: pass; generated smart triage evidence and child issue split material
date: 2026-05-08
```

Remaining risks:

- After issue 5476 advances, the representative may expose spread/rest
  diagnostics or unsupported dynamic call spread behavior.
