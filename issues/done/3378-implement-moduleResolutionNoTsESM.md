---
id: 3378
title: "Implement Moduleresolutionnotsesm (audit reopened #3378)"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432, 5229]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Triage moduleResolutionNoTsESM across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case in `moduleResolutionNoTsESM`. Fresh triage
now parses the `export default` sections and default imports. The current first
actionable blocker is module graph resolution for a local import such as
`import x from "./x.ts";`: the resolver looks on disk instead of resolving the
sibling virtual `// @filename: x.ts` section.

Problem: this generated bucket is superseded by existing virtual
`@filename` import resolution issue `5229`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolutionNoTsESM.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionNoTsESM.ts --detail
```

## Desired final state

This generated bucket is closed. The actionable first blocker is tracked by
issue `5229`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede with existing implementation-ready issue `5229`
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the superseding issue note

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
- [x] Superseding issue `5229` contains exact `reference-triage` ownership evidence
- [x] Superseding issue includes failing path, diagnostic code, source context, and parser/TypeScript AST evidence
- [x] Superseding issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionNoTsESM.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolutionNoTsESM.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; this close only moves a
  generated triage bucket and updates issue metadata, with no Rust source
  changes.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none; superseded by existing issue `5229`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionNoTsESM.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated manually on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-coverage tsc \
  --path-filter reference/typescript/tests/cases/compiler/moduleResolutionNoTsESM.ts \
  --detail --no-dashboard-data
```

Result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
per-file: moduleResolutionNoTsESM.ts => UnsupportedModule / import-export
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/moduleResolutionNoTsESM.ts
```

Observed:

```text
triage headline: UnresolvedName `x`
resolved/module_graph: issue-232 missing local module `./x.ts`
message: tried .../reference/typescript/tests/cases/compiler/./x.ts at 265..273
source:
// @filename: x.ts
export default 0;

// @filename: user.ts
import x from "./x.ts";
```

Compiler evidence:

```text
tokens: ok for export default sections and default imports
ast: ok; ExportDefault nodes for x.ts, y.tsx, z.d.ts and ImportDefault nodes
resolved: module_graph fails before extension-resolution semantics because
the virtual @filename section x.ts is not registered as a local module
TypeScript oracle: TS2307 diagnostics for explicit .ts/.tsx/.d.ts imports and
extensionless suggested-fix imports
```

Existing issue `5229` exactly covers resolving local imports between virtual
`@Filename` / `@filename` sections before later module-resolution diagnostics
become actionable.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionNoTsESM.ts --detail --no-dashboard-data
result: pass; reproduced UnsupportedModule/import-export first blocker
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionNoTsESM.ts
result: pass; first actionable blocker is virtual @filename local import resolution, superseded by issue 5229
date: 2026-05-08
```

Remaining risks:

- After issue `5229`, this reference may advance to the intended TS2307
  behavior for explicit `.ts`, `.tsx`, and `.d.ts` import specifiers under
  ESM, or to duplicate default export diagnostics.

## Close note

Superseded by issue `5229`. Fresh triage reaches module graph construction and
fails to resolve `./x.ts` to the sibling virtual `x.ts` section before ESM
no-TS-extension diagnostics are actionable.

superseded-by: 5229

## Reopened by audit

Date: 2026-05-05

Classification: must-reopen.

Reopen reason: frontmatter still says `class: blocked`, which is incompatible with a completed issue unless explicit supersedence/closure evidence is present.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/done/3378-implement-moduleResolutionNoTsESM.md` before this move
- `issues/done/3378-implement-moduleResolutionNoTsESM.md` after this move

Split follow-up: none created; existing issue `5229` is the tracking item for
the current first blocker.
