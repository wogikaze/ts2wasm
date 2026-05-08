---
id: 3332
title: "Implement Moduledeclarationexportstarshadowingglobalisnameable"
type: spike
area: frontend/syntax
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated parser-syntax bucket as superseded by the virtual
`@filename` local re-export resolution issue:
`issues/open/5229-resolve-imports-between-filename-sections.md`.

## Problem

Fresh triage shows the current first blocker is not a standalone parser bucket.
The module graph stops on the first virtual re-export source:

```text
issue-232: missing local module `./account` re-exported from model/index.ts; tried model/./account.ts, model/./account.js at 14..25
```

The later `declare global`, `import * as model from "./model"`, and declaration
nameability diagnostics are not reachable until virtual `export * from
"./account"` resolves to the sibling `@filename` section.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleDeclarationExportStarShadowingGlobalIsNameable --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedTypeScriptSyntax:1
unsupported_features=parser-syntax:1
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleDeclarationExportStarShadowingGlobalIsNameable.ts
```

Source shape:

```ts
// @filename: model/index.ts
export * from "./account";

// @filename: model/account.ts
export interface Account {
    myAccNum: number;
}
interface Account2 {
    myAccNum: number;
}
export { Account2 as Acc };

declare global {
    interface Account {
        someProp: number;
    }
    interface Acc {
        someProp: number;
    }
}

// @filename: user.ts
import * as model from "./model";
export const func = (account: model.Account, acc2: model.Acc) => {};
```

Compiler evidence:

```text
tokens: ok through export * from "./account", exported interfaces, declare global, import "./model", and exported const
triage: module_graph reports issue-232 missing local module ./account for the re-export source
ast/resolved dump: also exposes later issue-400 ambient global declaration boundary at declare global
```

TypeScript oracle evidence:

```text
TS2307: Cannot find module './account' or its corresponding type declarations.
TS2307: Cannot find module './model' or its corresponding type declarations.
```

## Desired final state

Implement the first blocker in
`issues/open/5229-resolve-imports-between-filename-sections.md`.

## Scope

In scope:

- [x] Confirm the generated bucket's current evidence.
- [x] Match the current first blocker to an existing implementation-ready issue.
- [x] Preserve exact reproduction commands and later diagnostics.

Out of scope:

- Direct implementation from this generated bucket.
- Ambient `declare global` support.
- Declaration emit/nameability diagnostics after module graph resolution.

## Affected paths

Expected implementation owner:

- `crates/compiler/src/module_graph.rs`
- `crates/compiler/src/lib.rs`
- focused compiler tests or fixtures

Do not touch from this bucket:

- backend/runtime emit
- unrelated frontend syntax

## Acceptance criteria

- [x] Superseding issue identified: `issues/open/5229-resolve-imports-between-filename-sections.md`.
- [x] Exact reproduction commands and observed diagnostics are recorded.
- [x] The generated bucket no longer remains as a stale blocked parser-syntax issue.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleDeclarationExportStarShadowingGlobalIsNameable --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleDeclarationExportStarShadowingGlobalIsNameable.ts
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

- [x] existing issue 5229 owns the first blocker

## Notes

Superseded by
`issues/open/5229-resolve-imports-between-filename-sections.md`.

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

- After issue 5229 resolves `./account` and `./model`, this case may expose
  ambient `declare global` or declaration nameability diagnostics.
