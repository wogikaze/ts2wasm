---
id: 1224
title: "Implement Classmemberinitializerwithlamdascoping Module System Amd"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1224.

## Summary

Triage classMemberInitializerWithLamdaScoping-module-system-amd across 3
reference test cases and close the stale generated bucket with current
smart-triage evidence.

## Problem

Reference test results previously showed 3 cases failing in directory
`classMemberInitializerWithLamdaScoping-module-system-amd` with diagnostics:
module-system-amd. Fresh triage shows two cases now build successfully and one
case exposes a narrower constructor FuncId invariant.

Problem: the generated bucket is no longer a coherent module-system AMD work
item. `classMemberInitializerWithLamdaScoping.ts` and
`classMemberInitializerWithLamdaScoping5.ts` are build passes, while
`classMemberInitializerWithLamdaScoping2.ts` needs an invariant-specific owner.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm no module-system AMD first blocker remains for two paths
- [x] Split the remaining invariant into child issue 5325
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence

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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded/split
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping5.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping5.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5325-fix-multifile-class-constructor-funcid-invariant.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping.ts`
- `reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts`
- `reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping5.ts`

## Duplicate detection

- `issues/done/5247-fix-js-noemit-class-constructor-funcid-invariant.md` - related same invariant in a JS/noEmit path, not exact for this multi-file TypeScript reference window

## Smart triage

Fresh triage shows this generated module-system bucket is stale.

### Smart triage: Build pass: classMemberInitializerWithLamdaScoping

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping.ts
```

Coverage result:

```text
build_pass=1
unsupported=0
```

TypeScript oracle reports TS2403 and TS2301 semantic diagnostics, but ts2wasm
has no current build blocker for this path.

### Smart triage: classMemberInitializerWithLamdaScoping2

- Issue class: `triage-needed`
- Feature label: `invariant-violation`
- Diagnostic: `InvariantViolation` / `compiler-invariant`
- Current compiler message: `ClassDecl constructor FuncId 0 is out of range (program has 0 function(s))`
- Path: `reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts --detail --no-dashboard-data
```

Coverage result:

```text
build_pass=1
unsupported=0
```

Compiler evidence:

```text
tokens: ok
ast: ok; Let field1 plus ClassDecl Test1 with constructor parameter property
resolved: ok; ClassDecl Test1 has constructor Some([...])
triage: InvariantViolation ClassDecl constructor FuncId 0 is out of range
```

Split result:

- `issues/open/5325-fix-multifile-class-constructor-funcid-invariant.md`

### Smart triage: Build pass: classMemberInitializerWithLamdaScoping5

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping5.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping5.ts
```

Coverage result:

```text
build_pass=1
unsupported=0
```

## Completion evidence

Commits:

- Split to `issues/open/5325-fix-multifile-class-constructor-funcid-invariant.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; BuildPass
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping2.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; invariant split to issue 5325
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping5.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; BuildPass
date: 2026-05-07
```

Remaining risks:

- Build-pass paths still have TypeScript oracle semantic diagnostics (TS2403,
  TS2301) that are not enforced by the current build coverage window.
