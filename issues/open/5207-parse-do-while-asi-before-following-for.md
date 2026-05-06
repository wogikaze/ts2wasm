---
id: 5207
title: "Parse do-while ASI before following for"
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

Accept a `do { ... } while (cond)` statement without an explicit semicolon when
the next statement is `for (...) { ... }`.

## Problem

Problem: capturedLetConstInLoop parser tests currently fail because the parser
expects an explicit semicolon after `do ... while (...)` and rejects the
following `for`.

Current diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(For)
```

TypeScript accepts these files with no diagnostics.

## Current failure

Use the validation commands below to reproduce the two representative failures.

Observed failures:

```text
capturedLetConstInLoop1.ts: expected Semicolon, got Some(For) at 487..490
capturedLetConstInLoop1_ES6.ts: expected Semicolon, got Some(For) at 446..449
```

Source shape:

```text
} while (1 === 1)

for (let y = 0; y < 1; ++y) {
    let x = 1;
    (() => x);
}
```

Parser evidence:

```text
tokens: ok
ast: UnsupportedSyntax expected Semicolon, got Some(For)
resolved: same parser failure
```

## Desired final state

The parser treats the semicolon after `do ... while (...)` as optional under
ASI rules and resumes statement parsing at the following `for`.

## Scope

In scope:

- [ ] Accept the exact no-semicolon `do ... while (...)` followed by `for`
  statement pattern.

Out of scope:

- Unrelated parser-syntax failures in capturedLetConstInLoop2/4/5/12/13.
- Resolver or lowering behavior after this parser failure is cleared.

## Affected paths

Expected:

- `crates/frontend/src/parser.rs`
- focused parser/compiler fixtures

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`

## Acceptance criteria

- [ ] The two affected reference triage commands no longer report
  `expected Semicolon, got Some(For)` at the post-`do while` boundary.
- [ ] A focused parser fixture proves `do { } while (expr)` followed by
  `for (...) {}` parses without an explicit semicolon.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop1_ES6.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/capturedLetConstInLoop1.ts --detail
```

Not run:

- none

## Completion evidence

Fill when implemented.
