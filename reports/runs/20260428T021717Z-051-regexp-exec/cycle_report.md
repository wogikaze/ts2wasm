# Cycle Report: issue 051 RegExp exec

Run ID: `20260428T021717Z-051-regexp-exec`
Branch: `agent/051-regexp-exec-20260428T020949Z`
Outcome: `PROGRESS`

## Scope

Implemented one constrained `RegExp.prototype.exec` continuation slice for the existing plain byte pattern subset.

- `/plain/.exec(input)` lowers to the existing `RegExpMatch` runtime helper.
- `let r = new RegExp("plain"); r.exec(input)` lowers to the same helper through identifier-backed RegExp receiver tracking.
- Hit behavior returns the matched string for current stringification coverage.
- Miss behavior returns `null`.
- Unsupported metacharacter patterns such as `/a*/.exec("aaa")` still produce an `issue-051` diagnostic.

This does not claim full JavaScript match-array semantics.

## Parser Scope Decision

An attempted direct `new RegExp("abc").exec("zabcx")` IR/fixture case reproduced a parser limitation:

```text
Diagnostic { code: UnsupportedSyntax, message: "expected Semicolon, got Some(Dot)" }
```

The final slice keeps constructor-backed coverage through an identifier receiver, which is already supported by the current parser and lowering model.

## Validation

```text
cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 17 tests run, 17 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 14 tests run, 14 passed

node fixtures/core-semantics/regexp-test.ts
result: pass; stdout:
true
false
true
true
false
abc
null
needle
true
abc
true
needle
true
plain

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-regexp-exec.wasm && iwasm /tmp/ts2wasm-issue051-regexp-exec.wasm
result: pass; stdout matched Node stdout

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass

cargo nextest run
result: not run; skipped because this slice only routes RegExp.prototype.exec to the existing RegExp-only RegExpMatch helper and does not alter shared non-RegExp runtime behavior.
```

## Remaining Work

Issue 051 remains open. Full match-array semantics, broader RegExp syntax/flags/state, and parser support for direct member access after `new RegExp(...)` remain outside this progress slice.
