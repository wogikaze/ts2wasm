---
id: 3344
title: "Implement Modulekeyworddeprecated"
type: maintenance
area: frontend/syntax
class: superseded
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed as a stale generated bucket. Fresh reference coverage for
`moduleKeywordDeprecated.ts` now reports `build_pass`; the remaining direct
triage dump path reports the unspanned multi-section empty-body guard already
owned by `issues/open/5187-lower-namespace-only-multi-section-files.md`.

## Problem

Reference test results previously showed 1 case failing in directory
`moduleKeywordDeprecated` with diagnostics: import-export. Fresh coverage no
longer reproduces a build blocker:

```text
reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts: build_pass
```

The direct smart-triage dump command still reports:

```text
UnsupportedSyntax: multi-section file has no module bodies
```

That empty-body guard is not specific to this generated bucket and is already
tracked by issue 5187.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] Fresh coverage proves the generated failing bucket is no longer a current build blocker
- [x] Remaining direct-triage empty-body behavior is covered by `issues/open/5187-lower-namespace-only-multi-section-files.md`
- [x] This closed bucket preserves the exact reference path, diagnostic, source context, token evidence, and TypeScript oracle diagnostics

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5187-lower-namespace-only-multi-section-files.md` for the direct-triage empty-body guard

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts`

## Duplicate detection

- none found by path/title/feature scan

## Smart triage

Fresh coverage on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts --detail --no-dashboard-data
```

Result:

```text
executed=1
build_pass=1
unsupported=0
reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts: build_pass
```

Fresh direct triage on 2026-05-08:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts
```

Result:

```text
UnsupportedSyntax: multi-section file has no module bodies
```

Source context:

```text
// @target: es2015
// @filename: foo.ts

// Error
module notok { }
module not.ok { }
declare module bad { }
declare module also.bad { }
```

Compiler evidence:

```text
tokens: ok; `module` and `declare` are tokenized as identifiers, with dotted names and string ambient module names preserved
ast: ok but empty
resolved: ok but empty
visible symbols: []
```

TypeScript oracle diagnostics:

```text
TS1540: A 'namespace' declaration should not be declared using the 'module' keyword. Please use the 'namespace' keyword instead.
TS2664: Invalid module name in augmentation, module 'good' cannot be found.
TS2664: Invalid module name in augmentation, module 'alsogood' cannot be found.
```

Superseded by:

- `issues/open/5187-lower-namespace-only-multi-section-files.md`

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleKeywordDeprecated.ts
result: pass; direct-triage dump path reports multi-section empty-body guard covered by issue 5187
date: 2026-05-08
```

Remaining risks:

- When semantic diagnostics become part of the coverage gate, this reference
  may need a narrower diagnostic issue for TS1540 module-keyword deprecation and
  TS2664 ambient module augmentation errors.
