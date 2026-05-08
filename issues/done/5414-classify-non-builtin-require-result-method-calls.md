---
id: 5414
title: "Classify non-builtin require result method calls"
type: feature
area: ir/lowering
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Classify method calls on locals initialized from non-builtin `require(...)`
instead of falling through to the generic issue-211 unknown receiver diagnostic.

## Problem

`moduleExportsTypeNoExcessPropertyCheckFromContainedLiteral.ts` parses and
builds AST for local CommonJS imports, but lowering rejects:

```ts
const tseslint = require('./typescript-eslint.js');
tseslint.config(eslintReact);
```

Current diagnostic:

```text
UnsupportedSyntax: issue-211: unknown receiver class for method `config`
```

Problem: the compiler does not distinguish a non-builtin CommonJS require result
from an ordinary unknown class receiver before method-call lowering.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExportsTypeNoExcessPropertyCheckFromContainedLiteral.ts
```

Evidence:

```text
tokens: ok through require calls and tseslint.config(...)
ast: ok; Call(Member(Ident("tseslint"), "config"), [Ident("eslintReact")])
resolved/lowered: issue-211 unknown receiver class for method `config`
```

TypeScript oracle reports missing Node globals (`module` and `require`), not an
error at the `tseslint.config(eslintReact)` member call itself.

## Desired final state

The compiler no longer reports generic unknown receiver class for this
non-builtin require-result method call. It should either emit a narrower
unsupported CommonJS package-resolution diagnostic or preserve enough require
result metadata to advance to the next reference blocker.

## Scope

In scope:

- [ ] Track `const name = require("./relative.js")` as a non-builtin CommonJS require result.
- [ ] Classify `name.method(...)` before generic issue-211 receiver lowering.
- [ ] Re-run the representative reference and record the next blocker.

Out of scope:

- Resolving or executing arbitrary CommonJS packages.
- Node ambient declarations for `module` and `require`.
- Builtin `require("fs")` alias handling, tracked by issue 5405.
- Interface-typed erased local method calls, tracked by issue 5222.

## Affected paths

Expected:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused CLI/IR tests

Do not touch:

- backend emit unless the narrowed diagnostic requires a lowering-only hook
- broad package/module resolution

## Acceptance criteria

- [ ] The representative reference no longer reports issue-211 unknown receiver class for `config`.
- [ ] A focused fixture covers `const pkg = require("./pkg.js"); pkg.config(arg);`.
- [ ] Existing `require("fs")` builtin behavior and interface-typed method-call diagnostics remain unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(require) or test(method)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExportsTypeNoExcessPropertyCheckFromContainedLiteral.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Docs / current-state / issue sync

Final-state docs: not affected.
Current state: not affected.
Follow-up issues: none.

## Notes

Split from generated bucket
`issues/done/3338-implement-moduleExportsTypeNoExcessPropertyCheckFromContainedLiteral.md`.

## Completion evidence

Fill only when implemented.
