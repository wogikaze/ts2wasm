---
id: 3442
title: "Implement Narrowbyinstanceof"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-01
status: done
---

## Summary

Closed after splitting the current `instanceof` RHS callable/prototype blocker
to `issues/open/5447-support-instanceof-callable-prototype-rhs.md`.

## Problem

Reference test results showed 1 case fail in directory `narrowByInstanceof`
with diagnostics: parser-syntax. Fresh triage shows tokens and AST now succeed;
the current blocker is issue-207 for an `instanceof` RHS parameter typed as a
callable/prototype object.

Problem: narrowByInstanceof had 1 generated reference failure and needed
smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByInstanceof.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByInstanceof.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
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
- [x] Child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/narrowByInstanceof.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/narrowByInstanceof.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5447-support-instanceof-callable-prototype-rhs.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/narrowByInstanceof.ts`

## Duplicate detection

- No existing open implementation-ready issue owned this callable/prototype
  `instanceof` RHS shape, so it was split to issue 5447.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByInstanceof.ts --detail --no-dashboard-data

result:
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Fresh triage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByInstanceof.ts

result:
UnsupportedSyntax: issue-207: instanceof right-hand side must be a supported class constructor `A`
```

Source context:

```ts
type AA = {
    (): void;
    prototype: A;
}

function foo(x: A | B | C, A: AA, B: BB, AB: AA | BB) {
    if (x instanceof A) {
        x;
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; `x instanceof A`, `x instanceof B`, and `x instanceof AB` are InstanceOf expressions
resolved/lowered: issue-207 for RHS A
TypeScript oracle: ok, diagnostics=[]
```

## Completion evidence

Closed after splitting issue 5447.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/narrowByInstanceof.ts --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/narrowByInstanceof.ts
result: pass; split to issue 5447 for callable/prototype instanceof RHS
date: 2026-05-08
```

Remaining risks:

- Later `instanceof` narrowing semantics and class-value runtime support may
  need separate follow-ups after issue 5447 advances this path.
