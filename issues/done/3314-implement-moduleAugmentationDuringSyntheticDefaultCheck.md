---
id: 3314
title: "Implement Moduleaugmentationduringsyntheticdefaultcheck"
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

Closed this generated `import-export` bucket as superseded by the open
implementation-ready issue 5346.

Fresh smart triage shows the first actionable blocker is `export = moment;` in
the virtual `node_modules/moment/index.d.ts` section. The parser stops before
AST construction with the generic issue-055 static export boundary.

## Problem

`moduleAugmentationDuringSyntheticDefaultCheck.ts` combines CommonJS export
assignment, package imports, ambient module augmentations for `moment` and
`moment-timezone`, and import-equals require. The current first blocker is much
narrower than the generated bucket title: the frontend does not yet parse
`export = expr;`.

Problem: this bucket is already owned by issue 5346 for CommonJS export
assignment parsing. Later package resolution and module augmentation behavior
should be re-triaged only after that parser boundary advances.

## Current failure

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDuringSyntheticDefaultCheck --detail --no-dashboard-data
```

Representative triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDuringSyntheticDefaultCheck.ts
```

## Desired final state

This generated bucket is closed. Implement the current first blocker from
`issues/open/5346-parse-commonjs-export-assignment-statements.md`.

## Scope

In scope:

- [x] Inspect fresh coverage and smart triage.
- [x] Identify `export = moment;` as the current first blocker.
- [x] Confirm open issue 5346 owns CommonJS `export = expr;` parsing.
- [x] Record later package/module augmentation risks.

Out of scope:

- Direct implementation from this generated bucket.
- Package resolution for `moment` or `moment-timezone`.
- Ambient module augmentation semantics.
- Synthetic default compatibility behavior.

## Affected paths

Expected:

- `issues/open/5346-parse-commonjs-export-assignment-statements.md`

Do not touch:

- Rust implementation files

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded.
- [x] Existing owner issue 5346 contains the exact `moduleAugmentationDuringSyntheticDefaultCheck.ts` evidence.
- [x] Closure preserves exact reproduction commands and current diagnostic.
- [x] Later TypeScript oracle diagnostics are recorded as follow-up risk, not mixed into this bucket.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDuringSyntheticDefaultCheck --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDuringSyntheticDefaultCheck.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleAugmentationDuringSyntheticDefaultCheck.ts`

## Duplicate detection

- `issues/open/5346-parse-commonjs-export-assignment-statements.md` owns the
  current first blocker: `export = moment;` reports generic issue-055 before
  AST construction.
- Later package resolution for `moment` / `moment-timezone` is outside this
  generated bucket closure and should be triaged after issue 5346 advances.

## Smart triage

Fresh coverage on 2026-05-08:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Source shape:

```ts
// @filename: node_modules/moment/index.d.ts
declare function moment(): moment.Moment;
declare namespace moment {
  interface Moment extends Object {
    valueOf(): number;
  }
}
export = moment;

// @filename: node_modules/moment-timezone/index.d.ts
import * as moment from 'moment';
export = moment;
declare module "moment" {
    interface Moment { tz(): string; }
}

// @filename: idx.ts
import * as _moment from "moment";
declare module "moment" { ... }
declare module "moment-timezone" { ... }

// @filename: idx.test.ts
import moment = require("moment-timezone");
```

Tokens include declarations, namespace `moment`, both export assignments,
package imports, ambient module augmentations, and import-equals require. AST
and resolved output stop before AST construction:

```text
error: [UnsupportedModule] issue-055: unsupported static export; module resolution and loading are not implemented at 287..293
```

TypeScript oracle reports later diagnostics including duplicate identifiers,
missing `moment` / `moment-timezone` package modules, and invalid augmentation
module names.

## Completion evidence

Commits:

- this local closure commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationDuringSyntheticDefaultCheck --detail --no-dashboard-data
result: pass; executed=1, unsupported=1, UnsupportedSyntax:1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationDuringSyntheticDefaultCheck.ts
result: pass; current first blocker is issue-055 static export for `export = moment;`, owned by issue 5346
date: 2026-05-08
```

Remaining risks:

- Advancing issue 5346 may expose package resolution, import-equals, ambient
  module augmentation, or TypeScript oracle diagnostics listed above.
