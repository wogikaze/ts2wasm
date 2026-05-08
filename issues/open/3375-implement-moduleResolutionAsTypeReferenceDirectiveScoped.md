---
id: 3375
title: "Implement Moduleresolutionastypereferencedirectivescoped"
type: maintenance
area: frontend/syntax
class: superseded
priority: P2
depends_on: [5007, 5423]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

> **Reopened by audit** (2026-05-06)
> Classification: false-done (blocked)
> Reason: relapsed false-done: reopened in df7621e3, re-closed without implementation. No implementation commits.
>
> True-done checklist:
> 1. Implementation commits in the repo that satisfy the acceptance criteria
> 2. Filled completion evidence section with commits and validation results
> 3. No relapsed false-done pattern (previously reopened but re-closed without evidence)

## Summary

Triage moduleResolutionAsTypeReferenceDirectiveScoped across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 case in
`moduleResolutionAsTypeReferenceDirectiveScoped`. Fresh triage confirms the
current first blocker is the shared declaration-file parser gap: virtual
`.d.ts` sections contain `export const NAME: number;`, and the frontend rejects
those ambient declarations as runtime const declarations without initializers.

Problem: this generated bucket is superseded by existing declaration-file
exported-const issue `5423`.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirectiveScoped.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirectiveScoped.ts --detail
```

## Desired final state

This generated bucket is closed. The actionable first blocker is tracked by
issue `5423`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Supersede with existing implementation-ready issue `5423`
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
- [x] Superseding issue `5423` contains exact `reference-triage` ownership evidence
- [x] Superseding issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirectiveScoped.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirectiveScoped.ts
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

- [x] none; superseded by existing issue `5423`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirectiveScoped.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Generated manually on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-coverage tsc \
  --path-filter reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirectiveScoped.ts \
  --detail --no-dashboard-data
```

Result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-resolution:1
per-file: moduleResolutionAsTypeReferenceDirectiveScoped.ts => UnsupportedSyntax / module-resolution
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirectiveScoped.ts
```

Observed:

```text
diagnosis: UnsupportedSyntax / parser-or-frontend-unsupported
message: const declarations require an initializer at 13..18
actual AST/resolved error: const declarations require an initializer at 263..268
source:
// @Filename: /a/types/dummy/index.d.ts
export const dummy: number;
```

Compiler evidence:

```text
tokens: ok for repeated virtual .d.ts sections containing
Export Const Ident(...) : Ident("number") ;
ast/resolved: fail on the first declaration-file exported const
TypeScript oracle: TS1155 diagnostics for uninitialized const declarations,
then TS2307 scoped module-resolution diagnostics for later imports
```

Existing issue `5423` exactly covers virtual `.d.ts` sections whose
`export const NAME: Type;` declarations need to parse as ambient exported
metadata instead of runtime const declarations.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- this close/supersedence commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirectiveScoped.ts --detail --no-dashboard-data
result: pass; reproduced UnsupportedSyntax/module-resolution first blocker
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirectiveScoped.ts
result: pass; first actionable blocker is declaration-file exported const parsing, superseded by issue 5423
date: 2026-05-08
```

Remaining risks:

- After issue `5423`, this reference may advance to scoped type-root and
  package/module resolution for `@scoped/*` and mangled `@types` names.


---

## ⚠️ False-done audit (re-opened from issues/open/)

**Why this was false-done**: This is a generated triage bucket issue. It was
created as a `class: blocked` spike with `depends_on` pointing to a parent
meta-issue (5004 or 5007). When the parent meta-issue was moved to
`issues/open/`, this child issue was dragged along without any implementation
or triage work. The `## Completion evidence` section is unfilled (commits
placeholder `...`, validation result empty). Zero implementation commits
reference this issue.

**True-done checklist** (all must pass):

1. **Triage the representative failure path**: Confirm it is superseded by an
   existing open/done issue OR split into implementation-ready child issues
   with exact reproduction commands.

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - Issue URL or child issue path documenting the triage outcome
   - Or: the exact failing reference path has a matching open/done issue
   - Or: the failing test case no longer reproduces the original diagnostic

## Close note

Superseded by issue `5423`. Fresh triage stops at the first virtual `.d.ts`
`export const dummy: number;` declaration before scoped type-root or
module-resolution behavior is actionable.

superseded-by: 5423
