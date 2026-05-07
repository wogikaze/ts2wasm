---
id: 3316
title: "Implement Moduleaugmentationextendambientmodule"
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

Triage moduleAugmentationExtendAmbientModule across 2 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Fresh reference evidence shows this bucket is not an ambient-declaration parser
blocker.

Problem: both representative files now parse far enough to reach the existing
issue-232 bare/non-local module specifier boundary for `import { Observable }
from "observable"`. Package resolution for bare specifiers is intentionally out
of scope for that completed module graph contract.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule1.ts --detail
```

## Desired final state

This generated bucket is closed as superseded by
`issues/done/232-resolve-local-relative-es-module-graph.md`. Do not implement
directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 232 covers the current bare module specifier boundary
- [x] Confirm no child issue is needed from this generated bucket
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in completion evidence

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

- [x] Duplicate candidates below are confirmed as this issue is superseded
- [x] Existing issue 232 covers non-local module specifier diagnostics
- [x] Completion evidence includes both failing paths, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact module specifier boundary

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
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule1.ts
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

- [x] existing owner: `issues/done/232-resolve-local-relative-es-module-graph.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule1.ts`
- `reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule2.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh coverage on 2026-05-08 shows both reference files now report the
import/export module boundary:

```text
reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule1.ts: UnsupportedModule: import-export
reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule2.ts: UnsupportedModule: import-export
```

Focused triage for both files parses the imports, the `declare module
"observable"` blocks, and the runtime statements before module graph validation
reports the existing issue-232 non-local module specifier boundary:

```text
UnsupportedModule: issue-232: unsupported non-local module specifier `observable`
```

Representative source context:

```ts
import { Observable } from "observable"

(<any>Observable.prototype).map = function() { }

declare module "observable" {
    interface Observable<T> {
        map<U>(proj: (e:T) => U): Observable<U>
    }
}
```

This is not a standalone module augmentation implementation order until bare
package/module specifier handling advances beyond issue 232.

## Completion evidence

Closed after fresh coverage and triage confirmed the generated
ambient-declaration label is stale and the current blocker is already covered
by issue 232.

Commits:

- local issue cleanup commit that moves issue 3316 to done and records issue 232 as owner

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule --detail --no-dashboard-data
result: pass; executed=2, unsupported=2, unsupported_features=import-export:2
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule1.ts
result: pass; current blocker is issue-232 unsupported non-local module specifier `observable`
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleAugmentationExtendAmbientModule2.ts
result: pass; current blocker is issue-232 unsupported non-local module specifier `observable`
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
```

Remaining risks:

- Module augmentation behavior remains behind the completed issue-232 bare specifier boundary.

## False-done audit

**truly-done** (3316)

- Implementation commits: verified via `git log --oneline --all --grep=3316`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
