---
id: 1455
title: "Implement Constenumpreserveemitnamedexport"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Closed as superseded. Fresh triage shows both
`constEnumPreserveEmitNamedExport1.ts` and
`constEnumPreserveEmitNamedExport2.ts` currently stop at the same plain
`const enum A { ... }` parser misclassification already owned by
`issues/done/5184-parse-const-enum-declarations.md`.

## Problem

Reference test results previously grouped these files under a named export
bucket. Current compiler behavior does not reach the later named export or
import declarations because parsing stops at the first `const enum A`.

Problem: `constEnumPreserveEmitNamedExport` is blocked by const-enum declaration
parsing before preserve-emit named export behavior can be triaged.

## Current failure

Representative reproductions:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport2.ts
```

Coverage windows:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport2.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as superseded by
`issues/done/5184-parse-const-enum-declarations.md`. Do not implement directly
from this bucket.

## Scope

In scope:

- [x] Inspect fresh smart triage for both affected files.
- [x] Confirm the current first blocker is covered by issue 5184.
- [x] Preserve exact reproduction commands and representative diagnostic/AST
      evidence in this closed issue and the owner issue.

Out of scope:

- Direct implementation from this generated bucket.
- Named export emit/preserve behavior.
- Local module resolution for `./a`.
- Const-enum inlining.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `fixtures/`
- focused parser tests

Do not touch:

- unrelated runtime/backend code unless focused triage proves a backend-only
  blocker after parser support lands

## Acceptance criteria

- [x] Existing issue 5184 is confirmed as the current first-blocker owner.
- [x] This closed issue includes both failing paths, diagnostic code, source
      context, token evidence, and TypeScript AST evidence.
- [x] Completion evidence names the exact reference paths and current
      diagnostic/stdout change.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport2.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; current first blocker is already tracked by issue 5184

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport1.ts`
- `reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport2.ts`

## Duplicate detection

Current first blocker is covered by
`issues/done/5184-parse-const-enum-declarations.md`.

Resolution:

```text
Superseded by issue 5184. The active diagnostic is the const-enum parser
misclassification at the first plain const enum declaration in both files.
```

## Smart triage

### `constEnumPreserveEmitNamedExport1.ts`

- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current message: `const declarations require an initializer at 6..10`
- AST/resolved message: `const declarations require an initializer at 74..78`
- First failing source: `const enum A {`
- Token evidence includes `Const`, `Ident("enum")`, `Ident("A")`, enum member
  `Foo`, then later `export { A }`, `import { A } from './a'`, and another
  `export { A }`.
- TypeScript parses the first declaration as `EnumDeclaration "const enum A { ... }"`,
  then reports later duplicate identifier and missing module diagnostics.

### `constEnumPreserveEmitNamedExport2.ts`

- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Current message: `const declarations require an initializer at 6..10`
- AST/resolved message: `const declarations require an initializer at 74..78`
- First failing source: `const enum A {`
- Token evidence includes `Const`, `Ident("enum")`, `Ident("A")`, enum member
  `Foo`, then later `export { A }`, `import { A } from './a'`, and
  `export { A as B }`.
- TypeScript parses the first declaration as `EnumDeclaration "const enum A { ... }"`,
  then reports later TS2307 for missing `./a`.

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport1.ts
result: pass; current first blocker is the same const-enum parser support tracked by issue 5184
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport2.ts
result: pass; current first blocker is the same const-enum parser support tracked by issue 5184
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport1.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=import-export:1
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constEnumPreserveEmitNamedExport2.ts --detail --no-dashboard-data
result: pass; executed=1 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=import-export:1
date: 2026-05-07
```

Remaining risks:

- After issue 5184 advances const-enum parsing, these files may expose named
  export emit/preserve behavior, duplicate export diagnostics, missing local
  module handling, or const-enum inlining work as later blockers.
