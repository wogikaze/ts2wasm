---
id: 3513
title: "Implement Nocircularityselfreferentialgetter"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed stale generated parser-syntax bucket for the `noCircularitySelfReferentialGetter` reference family. Fresh focused coverage now build-passes all 4 affected files, so no implementation child is needed for this bucket.

## Problem

Reference test results originally showed 4 cases failing in the `noCircularitySelfReferentialGetter` family with parser-syntax diagnostics. Fresh focused coverage on 2026-05-08 shows no current build blocker for this family.

Problem: stale generated blocker; the current compiler accepts the affected getter/object/type-only syntax in all representatives.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter2.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter --detail --no-dashboard-data
```

## Desired final state

This stale generated bucket is closed with focused evidence that all affected files now build-pass. Any future semantic parity work for circular getter type inference should be tracked separately from this parser-syntax bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close the stale bucket without a child because the focused reference window now build-passes
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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

- [x] Duplicate candidates below are confirmed as no-match or unnecessary because the bucket is stale
- [x] Closed issue contains an exact `reference-triage` command
- [x] Closed issue includes affected paths, current result, visible symbols, parser/TypeScript AST evidence, and TypeScript oracle evidence
- [x] Completion evidence names the exact reference family and current build-pass result

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter2.ts
```

Not run:

- `cargo fmt --all --check` / `cargo nextest run`: issue metadata-only closure; no Rust implementation changed.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter2.ts`
- `reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter1.ts`
- `reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter3.ts`
- `reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter4.ts`

## Duplicate detection

- Fresh focused coverage found no current compiler blocker for this generated bucket.
- No child issue was created because all 4 affected references now build-pass.

## Smart triage

Fresh focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter --detail --no-dashboard-data
result: pass; executed=4 build_pass=4 unsupported=0 blocked=0 semantic_enabled=0
files:
- reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter1.ts => build_pass
- reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter2.ts => build_pass
- reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter3.ts => build_pass
- reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter4.ts => build_pass
date: 2026-05-08
```

Representative triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter2.ts
result: BuildPass / pass
visible symbols: function string; binding Category; binding name
compiler evidence: tokens, AST, and resolved AST all succeed through type-only interface/type declarations, getter syntax, optional chaining, and export const.
TypeScript oracle: diagnostics none; binding Category has recursive ZodObject shape; binding name is string | undefined.
date: 2026-05-08
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- closing commit

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter --detail --no-dashboard-data
result: pass; executed=4 build_pass=4 unsupported=0 blocked=0 semantic_enabled=0
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noCircularitySelfReferentialGetter2.ts
result: BuildPass / pass
date: 2026-05-08
```

Remaining risks:

- Semantic execution is not enabled for this reference coverage path.
- This closure does not claim full TypeScript circular getter type-inference parity; it only removes the stale generated parser-syntax/build blocker.
