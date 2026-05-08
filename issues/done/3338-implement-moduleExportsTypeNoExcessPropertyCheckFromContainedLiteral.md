---
id: 3338
title: "Implement Moduleexportstypenoexcesspropertycheckfromcontainedliteral"
type: spike
area: reference/triage
class: split
priority: P2
depends_on: []
blocks: [5414]
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import/export bucket by splitting its current blocker into
`issues/open/5414a-classify-non-builtin-require-result-method-calls.md`.

## Problem

Fresh triage shows the file no longer stops at module export syntax. The parser
and AST accept the CommonJS `module.exports` assignments and local `require`
calls, then lowering stops at a method call on a local initialized from
non-builtin `require(...)`.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleExportsTypeNoExcessPropertyCheckFromContainedLiteral --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExportsTypeNoExcessPropertyCheckFromContainedLiteral.ts
```

Source shape:

```ts
module.exports = { config };

const eslintReact = require('./eslint-plugin-react.js');
const tseslint = require('./typescript-eslint.js');

tseslint.config(eslintReact);
```

Compiler evidence:

```text
tokens: ok through module.exports object literals, require calls, and tseslint.config(...)
ast: ok; `tseslint.config(eslintReact)` is represented as a member call
resolved/lowered: UnsupportedSyntax issue-211 unknown receiver class for method `config`
```

TypeScript oracle evidence:

```text
TS2591: Cannot find name 'module'.
TS2591: Cannot find name 'module'.
TS2591: Cannot find name 'require'.
TS2591: Cannot find name 'require'.
```

## Desired final state

Implement issue 5414 or re-triage after that issue lands. Do not implement from
this generated bucket.

## Scope

In scope:

- [x] Confirm the generated import/export blocker is stale.
- [x] Preserve exact reproduction commands and issue-211 method-call evidence.
- [x] Split the current blocker into a focused implementation issue.

Out of scope:

- Direct implementation from this generated bucket.
- Full CommonJS package resolution.
- Node ambient type definitions for `module` or `require`.

## Affected paths

Expected implementation owner:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused CLI/IR tests

## Acceptance criteria

- [x] Current blocker is represented in `issues/open/5414a-classify-non-builtin-require-result-method-calls.md`.
- [x] Existing nearby issues 5405 and 5222 are not exact owners for this shape.
- [x] The generated bucket no longer remains as a stale blocked import/export issue.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
git diff --cached --check
```

Reference commands already run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleExportsTypeNoExcessPropertyCheckFromContainedLiteral --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExportsTypeNoExcessPropertyCheckFromContainedLiteral.ts
```

Not run:

- `cargo fmt --all --check` (issue lifecycle only; no Rust changes)
- `cargo nextest run` (issue lifecycle only; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5414a-classify-non-builtin-require-result-method-calls.md`

## Notes

Split to issue 5414.

## Completion evidence

Commits:

- filled by commit

Validation result:

```text
command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-08

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-08

command: git diff --check
result: pass
date: 2026-05-08

command: git diff --cached --check
result: pass
date: 2026-05-08
```

Remaining risks:

- After issue 5414 lands, this case may expose Node ambient `module`/`require`
  diagnostics or the intended excess-property-check semantic behavior.
