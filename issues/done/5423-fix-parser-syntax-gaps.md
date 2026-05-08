---
id: 5423
title: "W2: Fix top test262 parser syntax gaps (UnsupportedSyntax)"
type: feature
area: frontend
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Fix the most frequently encountered UnsupportedSyntax patterns in the test262 corpus at full scale. After the 5419 metadata fix, UnsupportedSyntax is the #1 blocker at 26,294 files.

## Problem

At full corpus (53,449 files), 26,294 files hit UnsupportedSyntax — the parser encounters syntax it can't handle. This is the single largest blocker after the 5419 metadata fix.

Problem: 26,294 test262 files hit UnsupportedSyntax.

## Top syntax gaps (estimated)

Based on feature breakdown (regexp-literal: 813, eval: 736, object-literal: 619, function: 416):
- RegExp literal edge cases (complex patterns, Unicode escapes)
- eval-related syntax forms
- Object literal edge cases (computed keys, spread, getter/setter)
- Function syntax variants (generators, async function expressions)
- Block-level function declarations (Annex B)
- Sequence expressions
- `with` statements

## Desired final state

- Most common UnsupportedSyntax patterns handled by the parser
- UnsupportedSyntax count reduced from 26,294 by at least 1,000 at full corpus
- Each fixed pattern has a build_smoke fixture

## Scope

In scope:

- [ ] Identify most common UnsupportedSyntax patterns from test262 detail output
- [ ] Fix at least 5 distinct syntax patterns
- [ ] Add build_smoke fixtures for each fixed pattern
- [ ] Run full corpus to measure reduction

Out of scope:

- Runtime semantics for these syntax forms (separate issues)
- Non-test262 syntax features (JSX, decorators)
- TypeScript-specific syntax

## Affected paths

Expected:

- `crates/frontend/src/parser/expressions.rs` — expression form parsing
- `crates/frontend/src/parser/statements*.rs` — statement form parsing
- `fixtures/basics-syntax/` — new fixture files (create dir if needed)

Do not touch:

- `crates/ir/` — IR out of scope
- `crates/backend-wasm/` — runtime out of scope
- `scripts/` — scripts out of scope

## Acceptance criteria

- [ ] At least 5 distinct syntax patterns that previously produced UnsupportedSyntax now parse without diagnostic
- [ ] Each pattern has a build_smoke fixture
- [ ] `mise run reference-coverage -- test262` shows UnsupportedSyntax decreased

## Validation

```sh
cargo fmt --all --check
cargo nextest run
mise run reference-coverage -- test262
```
