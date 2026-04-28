# Cycle report: issue 051 direct RegExp constructor exec

Agent: 051-regexp-new-exec-20260428T033000Z
Branch: agent/051-regexp-new-exec-20260428T033000Z
Worktree: /home/wogikaze/wgkz/ts2wasm-051-regexp-new-exec-20260428T033000Z
Implementation commit: 88ec4d6
Outcome: PROGRESS

## Scope

Implemented the assigned continuation slice for direct
`new RegExp("plain").exec(...)` receivers. This does not claim full RegExp
or match-array semantics; the existing plain byte-pattern subset and
issue-051 diagnostics remain in force.

## Changes

- Reproduced the pre-fix parser gap: direct constructor receiver member access
  failed with `expected Comma, got Some(Dot)`.
- Let parsed `new ...(...)` expressions continue through the existing
  member/index/call suffix parser.
- Added IR lowering coverage for direct `new RegExp("abc").exec("zabcx")`.
- Added diagnostic coverage for unsupported direct
  `new RegExp("a*").exec("aaa")`.
- Extended `fixtures/core-semantics/regexp-test.ts` with direct constructor
  `.exec` hit and miss cases.

## Validation

```text
cargo run -p ts2wasm-cli -- build /tmp/ts2wasm-051-direct-new-regexp-exec.ts -o /tmp/ts2wasm-051-direct-new-regexp-exec.wasm
result: fail before fix; parser rejected direct new RegExp receiver with `expected Comma, got Some(Dot) at 36..37`

cargo fmt --all --check
result: pass

cargo nextest run -E 'test(regexp)'
result: pass; 19 tests run, 19 passed

cargo nextest run -p ts2wasm-cli regexp
result: pass; 16 tests run, 16 passed

node fixtures/core-semantics/regexp-test.ts
result: pass; stdout true / false / true / true / false / abc / null / needle / true / abc / true / needle / true / plain / plain / true

cargo run -p ts2wasm-cli -- build fixtures/core-semantics/regexp-test.ts -o /tmp/ts2wasm-issue051-direct-new-regexp-exec.wasm && iwasm /tmp/ts2wasm-issue051-direct-new-regexp-exec.wasm
result: pass; stdout matched Node stdout: true / false / true / true / false / abc / null / needle / true / abc / true / needle / true / plain / plain / true

scripts/manager fmt
result: pass

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass
```

## Remaining work

Issue 051 remains open. Remaining acceptance areas include full RegExp literal
coverage, complete RegExp operations, full `exec` match-array semantics,
String.prototype.match completeness, flags/state behavior, and broader RegExp
syntax.
