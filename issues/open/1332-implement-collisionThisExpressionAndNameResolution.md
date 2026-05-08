---
id: 1332
title: "Implement Collisionthisexpressionandnameresolution"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1332.

## Summary

Closed by splitting the current object type declaration parser boundary into `issues/done/5342-preserve-class-after-object-type-declaration.md`.

## Problem

Fresh triage confirms this generated bucket is too broad for direct implementation. The current first blocker is a parser boundary in the leading TypeScript-only object type declaration:

```ts
var console : {
    log(message: any);
}
class Foo {
    ...
}
```

The parser reports `UnsupportedTypeScriptSyntax: unterminated TypeScript type annotation at 440..441` and never reaches the class body containing `return x => this`. The focused implementation work is now tracked by issue 5342.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the current observable parser boundary into child issue 5342
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue and this closure

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
- [x] Child issue 5342 contains an exact `reference-triage` command
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected: issue metadata only

Follow-up issues:

- [x] created `issues/done/5342-preserve-class-after-object-type-declaration.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts`

## Duplicate detection

- `issues/done/5339-preserve-var-after-object-type-declaration.md` is related but owns an object type declaration followed by another `var` declaration.
- `issues/done/5340-preserve-function-after-object-type-declaration.md` is related but owns an object type declaration followed by a `function` declaration.
- No existing issue owns the object type declaration followed by a `class` declaration shape, so issue 5342 was created.

## Smart triage

Generated 2026-05-07.

Command:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts
```

Result:

```text
Smart triage: Triage parser syntax: collisionThisExpressionAndNameResolution
Feature label: parser-syntax
Diagnostic: UnsupportedTypeScriptSyntax / unsupported-feature-boundary
Message: unterminated TypeScript type annotation at 440..441
Failure location: line 15, column 2, at the final class closing brace
```

Source context:

```text
12 |             return x => this;   // New scope.  So should inject new _this capture into function inner
13 |         }
14 |     }
15 | }
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts --detail --no-dashboard-data
```

```text
executed=1
build_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-resolution:1
semantic_enabled=0
```

Compiler evidence:

- Tokens: ok through typed `var console`, `class Foo`, method `x`, local `_this`, nested `function inner`, `console.log(_this)`, and `return x => this`.
- AST/resolved: fail with `UnsupportedTypeScriptSyntax: unterminated TypeScript type annotation at 440..441`.
- Visible symbols before failure include `console`, class `Foo`, local `_this`, and function `inner`.
- TypeScript oracle parses the file, reports duplicate global `console` TS2403, and reports TS2683 for implicit `this` at `return x => this`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closure commit pending

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts
result: pass; reproduced object type declaration followed-by-class parser boundary and split issue 5342
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionThisExpressionAndNameResolution.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=0 unsupported=1 blocked=0 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=module-resolution:1
date: 2026-05-07
```

Remaining risks:

- The later lexical `return x => this` behavior remains unproven until issue 5342 advances past the object type declaration parser failure.
- Duplicate global `console` compatibility diagnostics are out of scope for this parser split.
