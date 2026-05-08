---
id: 1258
title: "Implement Clodulewithpriorinstantiatedmodule"
type: spike
area: frontend/syntax
class: blocked
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
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1258.

## Summary

Triage cloduleWithPriorInstantiatedModule across 1 reference test case and
split the remaining semantic diagnostic gap into an implementation-ready child
issue.

## Problem

Reference test results originally showed 1 case failing in directory
`cloduleWithPriorInstantiatedModule` with diagnostics: import-export. Fresh
focused coverage on 2026-05-07 shows the case now build-passes.

Problem: the stale build blocker is gone, but TypeScript oracle reports TS2434
because a non-ambient namespace declaration appears before the class it merges
with. That narrower semantic follow-up is split to issue 5330.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5330 for
the remaining namespace-before-class merge diagnostic parity.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

Out of scope:

- Direct implementation from this generated bucket
- Full declaration merge runtime lowering

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
- [x] At least one child issue contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts
```

Not run:

- `cargo fmt --all --check` (issue metadata only)
- `cargo nextest run` (issue metadata only)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5330-report-namespace-before-class-merge-diagnostic.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts`

## Duplicate detection

- `issues/open/771-implement-augmentedTypesModules.md` is related but remains a
  broad generated bucket with five files and older namespace/module ownership
  evidence.
- `issues/open/5329-report-class-namespace-duplicate-member-diagnostics.md` is
  related but covers duplicate member TS2300 diagnostics, not TS2434 declaration
  order.
- No exact open issue covered the narrow `namespace M { ... } class M {}` order
  diagnostic, so issue 5330 was created.

## Smart triage

Generated on 2026-05-07.

Fresh focused coverage:

```text
executed=1
build_pass=1
unsupported=0
reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts: build_pass
```

Fresh triage:

```text
### Smart triage: Build pass: cloduleWithPriorInstantiatedModule

- Issue class: none
- Feature label: build-pass
- Diagnostic: BuildPass / pass
- Path: reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts
```

Compiler evidence:

```text
tokens: ok through first namespace Moclodule, class Moclodule, and second namespace Moclodule
ast/resolved: ok; retained AST contains only ClassDecl Moclodule after namespace erasure
```

TypeScript oracle evidence:

```text
TS2434: A namespace declaration cannot be located prior to a class or function with which it is merged.
```

Source shape:

```ts
namespace Moclodule {
    export interface Someinterface {
        foo(): void;
    }
    var x = 10;
}

class Moclodule {
}

namespace Moclodule {
    export class Manager {
    }
}
```

Split child: `issues/open/5330-report-namespace-before-class-merge-diagnostic.md`.

## Completion evidence

Closed as split on 2026-05-07.

Commits:

- pending

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts
result: pass; current compiler build-passes, TypeScript oracle reports TS2434
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/cloduleWithPriorInstantiatedModule.ts --detail --no-dashboard-data
result: pass; executed=1, build_pass=1, unsupported=0
date: 2026-05-07
```

Remaining risks:

- Full declaration merge runtime lowering remains out of issue 5330 scope.
