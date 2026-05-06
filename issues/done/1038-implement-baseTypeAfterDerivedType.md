---
id: 1038
title: "Implement Basetypeafterderivedtype"
type: spike
area: backend-wasm
class: superseded
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-06
completed: 2026-05-06
status: done
---

## Summary

Triage baseTypeAfterDerivedType across 1 failing reference test case and split this generated bucket into an implementation-ready child issue.

## Problem

Fresh smart triage shows the original parser-syntax bucket is stale. The case now parses and resolves, then fails in backend WAT validation because emitted WAT references `$exception_pending` without declaring the runtime global.

Problem: `baseTypeAfterDerivedType` is not a standalone implementation order; it is superseded by issue 5155 for the exact backend runtime-link failure.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts
```

Direct build reproduction:

```sh
cargo run -q -p ts2wasm-cli -- build reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts -o /tmp/ts2wasm-1038-baseTypeAfterDerivedType.wasm
```

Current direct-build stderr excerpt:

```text
error: [BackendIo] wat2wasm failed
/tmp/ts2wasm-2-0.wat:753:21: error: undefined global variable "$exception_pending"
    (if (global.get $exception_pending)
                    ^^^^^^^^^^^^^^^^^^
```

## Desired final state

This generated bucket is closed as superseded by `issues/done/5155-fix-exception-pending-runtime-link-for-top-level-statements.md`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one observable backend WAT/runtime-link behavior into a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the child issue

Out of scope:

- Direct implementation from this generated bucket
- Broad multi-feature fixes without child issue split

## Affected paths

Expected:

- `crates/backend-wasm/src/runtime_link_plan.rs`
- `crates/backend-wasm/src/stmt_emit.rs`
- `crates/backend-wasm/src/runtime_fn.rs`
- `crates/backend-wasm/src/lib.rs`

Do not touch:

- parser/resolver code unless `reference-triage` proves the failure happens before runtime lowering

## Acceptance criteria

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Child issue 5155 contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact reference path and `wat2wasm` stderr change

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
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts
cargo run -q -p ts2wasm-cli -- build reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts -o /tmp/ts2wasm-1038-baseTypeAfterDerivedType.wasm
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no owned Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] completed: `issues/done/5155-fix-exception-pending-runtime-link-for-top-level-statements.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts`

## Duplicate detection

- No existing open issue matched the exact missing `$exception_pending` runtime-link/WAT-validity failure.

## Smart triage

### Smart triage: Triage backend io: baseTypeAfterDerivedType

- Issue class: `triage-needed`
- Feature label: `backend-io`
- Diagnostic: `BackendIo` / `backend-io`
- Path: `reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts
```

Source context:

```text
interface Derived extends Base {
    method(...args: any[]): void;
}

interface Base {
    method(...args: any[]): void;
}

class Derived2 implements Base2 {
    method(...args: any[]) {}
}
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "Derived2",
    "line": 10,
    "column": 1
  }
]
```

Parser/resolver evidence:

```text
tokens: ok
ast: ok; ClassDecl Derived2 with method `method(...args)`
resolved: ok; ClassDecl Derived2 method `method` has one rest parameter `args`
wat: generated, then final build fails in wat2wasm
```

TypeScript oracle evidence:

```text
ok: true
diagnostics: []
parameter hints: `args` has type `any[]` in the interfaces and class method
```

Resolution:

```text
Issue 5155 now owns the concrete backend runtime-link contract: top-level statement emission references `$exception_pending`, so emitted WAT must declare the runtime global or avoid the guard.
```

## Completion evidence

Commits:

- superseded by `issues/done/5155-fix-exception-pending-runtime-link-for-top-level-statements.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts
result: pass; reproduced current BackendIo/wat2wasm missing `$exception_pending` failure and split issue 5155
date: 2026-05-06

command: cargo run -q -p ts2wasm-cli -- build reference/typescript/tests/cases/compiler/baseTypeAfterDerivedType.ts -o /tmp/ts2wasm-1038-baseTypeAfterDerivedType.wasm
result: fail as expected; stderr reports undefined global variable "$exception_pending"
date: 2026-05-06
```

Remaining risks:

- none; issue 5155 is complete.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/done/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

