---
id: 399
title: "Define TypeScript parse, erase, and emit boundary contract"
type: spike
area: frontend
class: done
priority: P1
depends_on: []
blocks: [345,346]
created: 2026-05-01
updated: 2026-05-01
completed: 2026-05-01
---

## Summary

Define the compiler contract for TypeScript-only syntax: which constructs are parsed and erased before runtime lowering, which constructs affect emit or module shape, and which constructs remain unsupported with a TypeScript-specific diagnostic.

This issue exists because the current coverage buckets mix parser gaps, type-erasure gaps, declaration-emit gaps, module emit behavior, and runtime JavaScript semantics under broad parser/frontend unsupported diagnostics.

## Problem

The tsc/tsgo coverage matrix currently exposes TypeScript-specific failures as many generated buckets such as `type-alias`, `declaration-emit`, `ambient-declaration`, `class-accessor`, `module-system-amd`, `type-annotation`, `type-assertion`, `type-system`, and `typescript-directive`.

Problem: TypeScript parse/erase/emit failures do not have a single boundary contract, so generated issues can be misread as many independent runtime language features instead of a few frontend responsibilities.

## Current failure

Current coverage evidence:

```sh
mise run reference-coverage -- tsc --limit 200
mise run reference-coverage -- tsgo --limit 120
```

Broader confirmation evidence:

```sh
mise run reference-coverage -- tsc --limit 500 --no-web-ui
mise run reference-coverage -- tsgo --limit 166 --no-web-ui
```

Observed buckets:

- `tsc`: `parser-syntax:47`, `ambient-declaration:30`, `type-alias:23`, `import-export:21`, `class-accessor:16`, `module-system-amd:10`, `declaration-emit:8`, plus smaller TypeScript syntax buckets.
- `tsgo`: `import-export:20`, `declaration-emit:16`, `parser-syntax:16`, `module-resolution:10`, `jsx:8`, `type-system:7`, `decorator:4`, and related TS syntax buckets.
- Broader `tsc --limit 500`: `parser-syntax:184`, `import-export:78`, `ambient-declaration:29`, `arguments-object:21`, `unknown-unsupported:16`, `runtime-subset:15`, `class-accessor:14`, `type-alias:10`, `declaration-emit:5`.
- Full `tsgo --limit 166`: `import-export:57`, `parser-syntax:27`, `declaration-emit:9`, `module-resolution:6`, `unknown-unsupported:6`, `class:5`, `type-system:5`.

Existing child/bucket issues include:

- `345`: TypeScript type alias coverage for tsc.
- `346`: TypeScript declaration emit coverage for tsgo.
- `400`: TypeScript ambient declaration erasure and rejection boundary for the top `ambient-declaration` bucket.
- Generated tsc issues for ambient declarations, accessors, AMD/module declarations, type assertions, and related parser/frontend buckets.

Issues `345` and `346` depend on this issue so they cannot be treated as independent implementation-ready slices before this boundary contract exists.

## Desired final state

The repository has a documented TypeScript frontend boundary that classifies each TypeScript-only construct into one of these outcomes:

- parse and erase before runtime lowering;
- parse and preserve enough shape for JavaScript emit/module behavior;
- parse and reject with `UnsupportedTypeScriptSyntax`;
- defer to a dedicated runtime JavaScript feature issue when the blocker is not TypeScript-only syntax.

Generated coverage issues reference that contract instead of treating every TypeScript bucket as an unrelated language feature.

## Scope

In scope:

- [x] Inventory the tsc/tsgo unsupported feature labels that are TypeScript-only syntax or TypeScript emit behavior.
- [x] Define parse/erase/emit ownership for `type`, `interface`, `declare`, ambient declarations, type annotations, type assertions, decorators, parameter properties, JSX, enum, namespace/module declarations, and import/export forms.
- [x] Decide which labels should emit `UnsupportedTypeScriptSyntax` versus `UnsupportedModule` or `UnsupportedRuntimeSubset`.
- [x] Split or update child implementation issues for the top coverage buckets after the contract is written.

Out of scope:

- Implementing all TypeScript syntax support in this issue.
- Full TypeScript type checking parity.
- JavaScript runtime builtin semantics not caused by TypeScript syntax.

## Affected paths

Expected:

- `docs/05-compatibility-and-semantics.md`
- `docs/06-testing-and-coverage.md`
- `docs/12-coding-standard.md`
- `crates/frontend/src/`
- `crates/ir/src/`
- `scripts/run/reference-coverage.py`
- `issues/open/`

Do not touch:

- `crates/backend-wasm/` unless the design proves an emit boundary needs backend representation.
- `crates/runtime-abi/`

## Acceptance criteria

- [x] A numbered docs section defines TypeScript parse/erase/emit categories and diagnostic ownership.
- [x] The tsc/tsgo feature labels listed in the current failure section are mapped to owner categories.
- [x] `UnsupportedTypeScriptSyntax`, `UnsupportedModule`, and `UnsupportedRuntimeSubset` usage is documented for reference coverage.
- [x] Child implementation issues are created or updated for at least the top three TypeScript-only buckets after duplicate review.
- [x] Docs/current-state/issues are synchronized when status or design changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
mise run update-issue-index
mise run check issue-index
mise run check issues
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 200
mise run reference-coverage -- tsgo --limit 120
```

Not run:

- `mise run reference-coverage -- tsc --limit 200` (not rerun; issue 399 is a docs/design contract close and uses the existing evidence recorded above)
- `mise run reference-coverage -- tsgo --limit 120` (not rerun; issue 399 is a docs/design contract close and uses the existing evidence recorded above)

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/05-compatibility-and-semantics.md`
- [x] updated: `docs/06-testing-and-coverage.md`
- [x] updated: `docs/12-coding-standard.md`

Current state:

- [x] updated: `current-state.md` (repo root) because the coverage interpretation now maps TypeScript-only buckets to boundary owners

Follow-up issues:

- [x] updated: issue 345 (`type-alias`)
- [x] updated: issue 346 (`declaration-emit`)
- [x] created: issue 400 (`ambient-declaration`)

## Notes

Use issues `345` and `346` as existing child examples, but do not unblock generated `triage-needed` buckets until representative `reference-triage` evidence confirms the exact parser or erasure blocker.

## Completion evidence

Commits:

- pending parent commit

Validation result:

```text
command: mise run reference-coverage -- tsc --limit 500 --no-web-ui
result: pass; unsupported feature buckets confirmed parser-syntax:184, import-export:78, ambient-declaration:29, type-alias:10, declaration-emit:5
date: 2026-05-01

command: mise run reference-coverage -- tsgo --limit 166 --no-web-ui
result: pass; unsupported feature buckets confirmed import-export:57, parser-syntax:27, declaration-emit:9, module-resolution:6
date: 2026-05-01

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK
date: 2026-05-01

command: mise run check issue-index
result: pass; issues/index.md queue OK and check_issue_health OK
date: 2026-05-01

command: mise run check issues
result: pass; issues/index.md queue OK and check_issue_health OK
date: 2026-05-01
```

Remaining risks:

- `cargo fmt --all --check` was not used as completion evidence in the parent worktree because unrelated unstaged Rust changes are present; this issue changes docs/issues only. Child implementation merges must run their own clean-worktree gates.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/399-define-typescript-parse-erase-emit-boundary.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
