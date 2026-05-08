---
id: 3578
title: "Implement Nodenextcjsnamespaceimportdefault"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5483, 5469]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Triage nodeNextCjsNamespaceImportDefault across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows both affected paths currently stop at dependency-module
named export validation for `export { d, a, ns }`.

Problem: the generated bucket mixes imported-binding export validation,
namespace-import export validation, later local virtual module resolution, and
NodeNext CJS/default interop. The first actionable blocker is split to issue
`5483`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts --detail
```

## Desired final state

This generated bucket is closed after splitting the current first blocker to
issue `5483`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split default-import binding named exports to issue `5483`
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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

- [x] Duplicate candidates below are confirmed as no-match, split, or superseded
- [x] Child issue `5483` contains exact `reference-triage` commands
- [x] Child issue includes failing paths, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference paths and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault2.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault2.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata-only split.
- `cargo nextest run`; issue metadata-only split.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5483-support-named-exports-of-imported-bindings-in-dependency-modules.md`
- [x] existing: `issues/open/5469-support-named-exports-of-namespace-imports-in-dependency-modules.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts`
- `reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault2.ts`

## Duplicate detection

- `issues/open/5469-support-named-exports-of-namespace-imports-in-dependency-modules.md` covers the related `ns` export-list binding if namespace imports become the next blocker, but does not cover the default-import local `d`.

## Smart triage

Fresh runs on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault2.ts
```

Focused coverage for both:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
```

Current headline/stack diagnostic for both:

```text
UnsupportedSyntax: issue-5005: dependency module `export { d }` references unknown local binding `d` at 69..70
```

Compiler evidence:

```text
tokens: ok through export const, optional export default, ImportDefaultNamed, ImportNamespace, ExportNamed, d.a, and ns.default.a
ast: ok; ExportDecl(Let a), optional ExportDefault("string"), ImportDefaultNamed local d/a, ImportNamespace local ns, ExportNamed d/a/ns, member expressions d.a and ns.default.a
resolved dump: later issue-232 missing local module `./a.cjs`
module build/stack: issue-5005 dependency-module named export unknown local binding `d`
visible symbols: []
```

TypeScript oracle:

```text
TS2323 cannot redeclare exported variable a
TS2395 merged declaration a must be all exported or all local
TS2307 cannot find module './a.cjs'
AST topLevel includes export const, optional export default, ImportDeclaration,
ImportDeclaration, ExportDeclaration, and member expressions d.a / ns.default.a.
```

## Completion evidence

Status: done

Commits:

- this local issue-cleanup commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts --detail --no-dashboard-data
result: pass; UnsupportedModule/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault2.ts --detail --no-dashboard-data
result: pass; UnsupportedModule/import-export
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault1.ts
result: pass; current dependency export-list unknown local binding `d` split to issue 5483
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/nodeNextCjsNamespaceImportDefault2.ts
result: pass; current dependency export-list unknown local binding `d` split to issue 5483
date: 2026-05-08
```

Remaining risks:

- After issue `5483`, the paths may expose namespace-import export-list
  support (`5469`), local virtual `./a.cjs` resolution, or NodeNext CJS/default
  interop.
