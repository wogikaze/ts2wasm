# Cycle Report: issue 054 Error instanceof

Status: PROGRESS
Issue: 054
Branch: agent/054-error-instanceof-20260428T025258Z
Worktree: /home/wogikaze/wgkz/ts2wasm-054-error-instanceof-20260428T025258Z

## Scope

Implemented the Error prototype identity and `instanceof` continuation slice for already-supported Error constructors: Error, TypeError, ReferenceError, and SyntaxError. Left issue 054 open because `.stack` and full close validation remain outstanding.

## Parent Worktree Correction

The parent reported that the first assignment artifact may have been created outside the assigned worktree. I stopped, verified `pwd` and branch, and recreated `reports/agents/054-error-instanceof-20260428T025258Z/assignment.md` inside the assigned worktree. A read-only status check of `/home/wogikaze/wgkz/ts2wasm` showed no stray tracked or untracked assignment artifact at that path during this cycle; I did not edit or clean the parent worktree.

## Pre-change Reproduction

Temporary fixture:

```ts
// @ts-nocheck
let generic = new Error("generic");
let type_error = new TypeError("type");
console.log(generic instanceof Error);
console.log(type_error instanceof TypeError);
console.log(type_error instanceof Error);
```

Node output:

```text
true
true
true
```

Pre-change compiler result:

```text
error: [UnsupportedSyntax] issue-207: instanceof right-hand side must be a supported class constructor `Error`
```

## Implementation

- Added `BuiltinErrorConstructor` and lowered built-in Error RHS identifiers in `instanceof` expressions to built-in prototype references.
- Changed built-in Error construction lowering from plain object literals to `ErrorNew`, preserving the existing `.message` behavior while assigning the correct prototype.
- Added backend globals and start-time initializers for built-in Error prototype objects. TypeError, ReferenceError, and SyntaxError prototypes chain to Error.prototype.
- Added GC root marking for built-in Error prototype globals.
- Added `fixtures/builtins-and-io/error-instanceof.ts` and a Node differential test.

## Direct Fixture Evidence

Command:

```sh
node fixtures/builtins-and-io/error-instanceof.ts > /tmp/ts2wasm-054-error-instanceof.node.out
cargo run -q -p ts2wasm-cli -- build fixtures/builtins-and-io/error-instanceof.ts -o /tmp/ts2wasm-054-error-instanceof.wasm
iwasm /tmp/ts2wasm-054-error-instanceof.wasm > /tmp/ts2wasm-054-error-instanceof.iwasm.out
diff -u /tmp/ts2wasm-054-error-instanceof.node.out /tmp/ts2wasm-054-error-instanceof.iwasm.out
```

Result: pass, no diff.

iwasm stdout:

```text
true
true
true
true
true
true
true
false
false
false
false
```

## Validation

```text
cargo check -p ts2wasm-backend-wasm -p ts2wasm-ir -p ts2wasm-cli
result: pass

cargo fmt --all --check
result: pass

cargo nextest run -E 'test(error)'
result: pass, 4 passed

cargo nextest run -p ts2wasm-cli error
result: pass, 2 passed

scripts/manager check-issue-health
result: pass

scripts/manager check-agent-state
result: pass
```

Full `cargo nextest run` was not run because this cycle reports PROGRESS, not DONE.

## Remaining Gaps

- `.stack` is still not implemented.
- Full issue close requirements, including full `cargo nextest run`, remain outstanding.
