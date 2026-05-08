---
id: 3382
title: "Implement Moduleresolutionwithmodule (audit reopened #3382)"
type: maintenance
area: compiler/multi-section
class: superseded
priority: P1
depends_on: [432, 5402]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage moduleResolutionWithModule across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh triage shows this generated bucket is blocked before
module-resolution-with-module behavior becomes reachable. The reference starts
with a virtual `node_modules/pkg/package.json` section, and the current
multi-section compiler path parses that JSON body as TypeScript source.

Problem: `moduleResolutionWithModule.ts` currently stops at the virtual
`package.json` property colon, which is already owned by issue 5402.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithModule.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithModule.ts --detail --no-dashboard-data
```

Observed result:

```text
UnsupportedSyntax: expected Semicolon, got Some(Colon) at 12..13
tokens: package.json object tokens, then `export declare function thing(): void;`, then `import * as p from "pkg";`
TypeScript AST top-level: Block for package.json body, FunctionDeclaration, ImportDeclaration, ExpressionStatement `p.thing();`
```

## Desired final state

This generated bucket is closed as superseded by
`issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`.
After issue 5402 lands, this reference may expose package `exports`, Node16 /
NodeNext package resolution, or declaration-file import behavior; those should
be tracked with fresh post-skip evidence.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede this bucket with a focused existing issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch:

- unrelated runtime/backend code unless `reference-triage` proves the failure is not frontend-owned

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Existing issue 5402 contains the implementation-ready package.json harness task
- [x] This close records failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Follow-up issue acceptance names the exact diagnostic/stdout change

## Validation

Required commands:

Issue-only close; Rust gates were not required for this lifecycle split.

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithModule.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithModule.ts
```

Not run:

- cargo fmt --all --check
- cargo nextest run

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionWithModule.ts`

## Duplicate detection

- `issues/open/5402-skip-package-json-filename-sections-in-reference-harness.md`
  matches the current package.json virtual-section parse boundary.

## Smart triage

Fresh triage:

```text
first_code_line: {
failure: UnsupportedSyntax expected Semicolon, got Some(Colon) at the package.json `"name"` property
visible symbols: []
tokens: package.json object tokens, then `export declare function thing(): void;`, then `import * as p from "pkg";`
ast/resolved: fail on the package.json property colon
TypeScript AST top-level: Block for package.json body, FunctionDeclaration `export declare function thing(): void;`, ImportDeclaration `import * as p from "pkg";`, ExpressionStatement `p.thing();`
```

Issue 5402 exactly covers virtual `package.json` sections being parsed as
module bodies instead of skipped or stored as metadata.

## Completion evidence

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionWithModule.ts --detail --no-dashboard-data
result: pass; one UnsupportedSyntax/module-resolution path reproduced, current blocker is package.json section parsing
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionWithModule.ts
result: pass; stops at virtual `node_modules/pkg/package.json` property colon, superseded by issue 5402
date: 2026-05-08
```

Remaining risks:

- After issue 5402 lands, this reference may expose package exports,
  Node16/NodeNext resolution, or declaration-file module graph blockers.
## Close note

Superseded by issue 5402, which owns virtual `package.json` sections being
parsed as executable source in the reference multi-section harness.

superseded-by: 5402

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/3382-implement-moduleResolutionWithModule.md` before this move
- `issues/done/3382-implement-moduleResolutionWithModule.md` after this move

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.
