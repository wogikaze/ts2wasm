---
id: 5295
title: "Resolve import-equals require to virtual node_modules class export"
type: feature
area: frontend/module-resolution
class: implementation-ready
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Resolve one reference shape: `/foo/test.ts` uses
`import myModule = require("myModule")`, and the same reference file provides
`/foo/node_modules/myModule/index.ts` with `export class c { }`.

## Problem

Current failure:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts
```

Result:

```text
UnsupportedModule: dependency module declaration export uses a form outside the current static export slice
module_graph detail: unsupported non-local module specifier `myModule`
```

Problem: bare `require("myModule")` aliases do not bind to a virtual
`node_modules/myModule/index.ts` section for `new myModule.c()`.

## Current failure

```ts
// @filename: /foo/node_modules/myModule/index.ts
export class c { }

// @filename: /foo/test.ts
import myModule = require("myModule");
new myModule.c();
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts --detail --no-dashboard-data
```

Result: `executed=1`, `build_pass=0`, `unsupported=1`, `blocked=0`,
`unsupported_diagcodes=UnsupportedModule:1`.

## Desired final state

The representative path advances past `myModule` lookup and dependency
`export class c { }`.

## Scope

In scope:

- [ ] Bind `require("myModule")` to `/foo/node_modules/myModule/index.ts`.
- [ ] Expose exported class `c` for `new myModule.c()`.
- [ ] Add one focused regression for this exact shape.

Out of scope:

- package.json fields
- import maps
- broad CommonJS emit
- tsconfig semantics

## Affected paths

Expected:

- `crates/frontend/src/`
- focused module-resolution tests or fixtures

Do not touch:

- unrelated backend code

## Acceptance criteria

- [ ] Triage no longer reports unsupported non-local module specifier `myModule`.
- [ ] Triage no longer reports unsupported dependency `export class c`.
- [ ] Focused coverage no longer reports `UnsupportedModule:1` for this boundary.
- [ ] A regression covers `import alias = require("pkg"); new alias.C();`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(module) or test(import) or test(require)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/compositeWithNodeModulesSourceFile.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from `issues/open/1403-implement-compositeWithNodeModulesSourceFile.md`.

## Completion evidence

Fill only when moving to `done`.
