---
id: 5210
title: "Parse do-while ASI before block end or expression"
type: feature
area: frontend/syntax
class: implementation-ready
priority: P2
depends_on: [5000]
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Accept `do { ... } while (cond)` without an explicit semicolon when the next
token is a closing block or a following expression statement.

## Problem

Problem: capturedLetConstInLoop parser tests still reject no-semicolon
`do ... while (...)` when the next statement boundary is `}` or `use(v)`.

Current diagnostics:

```text
UnsupportedSyntax: expected Semicolon, got Some(RightBrace)
UnsupportedSyntax: expected Semicolon, got Some(Ident("use"))
```

TypeScript accepts the representative syntax and advances to later semantic
diagnostics where applicable.

## Current failure

Use the validation commands below to reproduce the representative failures.

Observed failures:

```text
capturedLetConstInLoop2.ts: expected Semicolon, got Some(RightBrace)
capturedLetConstInLoop5.ts: expected Semicolon, got Some(Ident("use"))
```

Source shapes:

```text
do {
    let x;
} while (1 === 1)
}
```

```text
do {
    var v;
} while (1 === 1)

use(v);
```

## Desired final state

The parser treats the semicolon after `do ... while (...)` as optional under
ASI rules when the statement is followed by a block end or expression
statement.

## Scope

In scope:

- [ ] Accept the two representative post-`do while` ASI boundaries.

Out of scope:

- The post-`do while` before `for` case owned by issue 5207.
- Other capturedLetConstInLoop parser subfamilies.
- Resolver diagnostics that appear after parsing succeeds.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser/compiler fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `capturedLetConstInLoop2.ts` no longer reports `expected Semicolon, got
  Some(RightBrace)` at the no-semicolon `do while` boundary.
- [ ] `capturedLetConstInLoop5.ts` no longer reports `expected Semicolon, got
  Some(Ident("use"))` at the no-semicolon `do while` boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop5.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop2.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
