---
id: 5211
title: "Parse do-while ASI before labeled statement"
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
statement is a label followed by a `for` statement.

## Problem

Problem: capturedLetConstInLoop parser tests reject a no-semicolon
`do ... while (...)` when the next token starts a labeled statement.

Current diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Ident("l4"))
```

TypeScript accepts the representative syntax and parses the following labeled
loop normally.

## Current failure

Use the validation commands below to reproduce the representative failures.

Observed failures:

```text
capturedLetConstInLoop7.ts: expected Semicolon, got Some(Ident("l4")) at 1383..1385
capturedLetConstInLoop7_ES6.ts: expected Semicolon, got Some(Ident("l4")) at 1380..1382
```

Source shape:

```text
} while (1 === 1)

l4:
for (let y = 0; y < 1; ++y) {
    let x = 1;
}
```

## Desired final state

The parser treats the semicolon after `do ... while (...)` as optional under
ASI rules when the following statement is labeled.

## Scope

In scope:

- [ ] Accept the exact no-semicolon `do while` followed by labeled statement
  pattern.

Out of scope:

- The unlabeled post-`do while` before `for` case owned by issue 5207.
- Label-aware control-flow binding or resolver diagnostics after parsing.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser/compiler fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] `capturedLetConstInLoop7.ts` no longer reports `expected Semicolon, got
  Some(Ident("l4"))` at the no-semicolon `do while` boundary.
- [ ] `capturedLetConstInLoop7_ES6.ts` no longer reports `expected Semicolon,
  got Some(Ident("l4"))` at the same boundary.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop7.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop7_ES6.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop7.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
