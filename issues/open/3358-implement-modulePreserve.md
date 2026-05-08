---
id: 3358
title: "Implement Modulepreserve"
type: maintenance
area: frontend/syntax
class: split
priority: P1
depends_on: [432, 5324, 5402]
blocks: [5419, 5420]
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed by splitting the current uncovered parser blockers into child issues
5419 and 5420, and by mapping the remaining listed cases to existing owners.

## Problem

Reference test results show 5 listed cases fail in directory `modulePreserve`
with diagnostics: import-export. Fresh triage shows this bucket is not one
implementation slice:

- `modulePreserve1.ts`: dependency virtual-file `export class` issue-5005,
  owned by issue 5324.
- `modulePreserve2.ts`: virtual `package.json` JSON body parsed as source,
  owned by issue 5402.
- `modulePreserve3.ts`: parenthesized self-closing JSX expression parser
  boundary, split to issue 5419.
- `modulePreserve4.ts`: current first diagnostic is unresolved CommonJS
  `module`, matching TypeScript's TS2591 and the completed name-resolution
  behavior used by prior module export bucket closures.
- `modulePreserve5.ts`: static import attribute `with { type: "json" }`
  parser boundary, split to issue 5420.

Problem: this generated bucket mixes multiple unrelated blockers and must not
be implemented directly.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/modulePreserve1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserve1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split uncovered feature families into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue or closure mapping

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
- [x] Child issues 5419 and 5420 contain exact `reference-triage` commands
- [x] Child issues include failing path, diagnostic code, source context, visible symbols, parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands for this closure:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserve --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve2.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve3.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve4.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve5.ts
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Not run:

- Cargo gates; no Rust source changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5419a-parse-parenthesized-self-closing-jsx-expressions.md`
- [x] created: `issues/open/5420a-parse-import-attributes-with-clauses.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/modulePreserve1.ts`
- `reference/typescript/tests/cases/compiler/modulePreserve5.ts`
- `reference/typescript/tests/cases/compiler/modulePreserve2.ts`
- `reference/typescript/tests/cases/compiler/modulePreserve3.ts`
- `reference/typescript/tests/cases/compiler/modulePreserve4.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh run on 2026-05-08.

Coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserve --detail --no-dashboard-data
```

Coverage result:

```text
executed=7
build_pass=0
unsupported=7
unsupported_diagcodes=UnsupportedSyntax:7
unsupported_features=import-export:7
```

The path filter includes the 5 listed `modulePreserve1..5.ts` files plus
nearby `modulePreserveTopLevelAwait1.ts` and `modulePreserveImportHelpers.ts`,
which are tracked by separate generated buckets.

### modulePreserve1.ts

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve1.ts
```

Current diagnostic:

```text
UnsupportedModule: issue-5005: dependency module declaration export uses a form outside the current static export slice at 7..17
```

Evidence:

```text
source: `export class A {}` in virtual `/a.ts`
tokens: ok through `export class A {}`, `export = class B {}`, static import, import-equals, and re-export
ast/resolved dump stops later at `export = class B {}`, but the build stack trace confirms the first current blocker is dependency `export class A`
owner: issues/open/5324-support-dependency-export-class-declarations.md
```

### modulePreserve2.ts

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve2.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 174..175
```

Evidence:

```text
source: virtual package.json body with `"name": "dep"` and `"exports": { ... }`
tokens: JSON object tokens are present
ast/resolved: fail at first JSON property colon
TypeScript oracle: parses the package.json body as a Block and continues to later import/require diagnostics
owner: issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md
```

### modulePreserve3.ts

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve3.ts
```

Current diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: RightParen, span: Span { start: 276, end: 277 } }) at 277..278
```

Evidence:

```text
source: `(<div />);`
tokens: ok as LeftParen, Less, Ident("div"), Slash, Greater, RightParen, Semicolon
TypeScript oracle: AST contains ExpressionStatement `(<div />);`
split to: issues/open/5419a-parse-parenthesized-self-closing-jsx-expressions.md
```

### modulePreserve4.ts

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve4.ts
```

Current diagnostic:

```text
UnresolvedName: unresolved name: `module`
```

Evidence:

```text
source: `module.exports.y = 0;`
visible symbols: many imports and require-backed locals are visible later in the file
TypeScript oracle: TS2591 `Cannot find name 'module'`
mapping: same diagnostic handling used by done issue 3337 and completed name-resolution issue 056
```

### modulePreserve5.ts

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve5.ts
```

Current diagnostic:

```text
UnsupportedSyntax: expected Semicolon, got Some(Ident("with")) at 196..200
```

Evidence:

```text
source: `import data1 from "./data.json" with { type: "json" };`
tokens: static import tokens include Ident("with"), attribute object braces, Ident("type"), Colon, and String("json")
TypeScript oracle: AST contains ImportDeclaration with the import attribute clause
split to: issues/open/5420a-parse-import-attributes-with-clauses.md
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `pending`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/modulePreserve --detail --no-dashboard-data
result: pass; executed=7, unsupported=7, UnsupportedSyntax/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve1.ts
result: pass; maps to issue 5324
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve2.ts
result: pass; maps to issue 5402
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve3.ts
result: pass; split to issue 5419
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve4.ts
result: pass; current diagnostic is unresolved CommonJS `module`
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/modulePreserve5.ts
result: pass; split to issue 5420
date: 2026-05-08
```

Remaining risks:

- After issues 5324, 5402, 5419, and 5420 land, these references may expose
  later module-preserve emit parity, import-equals, dynamic import attributes,
  top-level await, JSON module resolution, or TS semantic diagnostic blockers.
