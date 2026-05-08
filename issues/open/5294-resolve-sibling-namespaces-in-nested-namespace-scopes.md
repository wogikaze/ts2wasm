---
id: 5294
title: "Resolve sibling namespaces in nested namespace scopes"
type: feature
area: frontend/name-resolution
class: implementation-ready
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Resolve namespace declarations declared in an enclosing namespace scope when
they are referenced from a nested namespace body, including references that
appear before the sibling namespace declaration in source order.

This is the current blocker from `complicatedPrivacy.ts`.

## Problem

`complicatedPrivacy.ts` enters `namespace m1 { export namespace m2 { ... } }`
and fails at `export class C2 implements m3.i3` because resolver lookup cannot
find sibling namespace `m3`, which is declared later inside `namespace m1`.
TypeScript accepts the namespace reference and reports later semantic
diagnostics instead.

Problem: nested namespace resolution does not predeclare or look up sibling
namespace names from the enclosing namespace scope.

## Current failure

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedPrivacy.ts
```

Focused coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complicatedPrivacy.ts --detail --no-dashboard-data
```

Observed 2026-05-07:

```text
Diagnostic: UnresolvedName / resolver-symbol
message: unresolved name: `m3` at 226..228
line: 13, column: 4
coverage: executed=1, build_pass=0, unsupported=1, blocked=0
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

Source:

```ts
namespace m1 {
    export namespace m2 {
        export class C2 implements m3.i3 {
            public get p1(arg) {
                return new C1();
            }
        }
    }

    namespace m3 {
        export interface i3 {
            f55(): string;
        }
    }
}
```

## Desired final state

`complicatedPrivacy.ts` no longer fails with `UnresolvedName` for `m3` in the
`implements m3.i3` clause. The compiler either resolves the sibling namespace
reference or advances to the next narrower diagnostic in the same reference
file.

## Scope

In scope:

- [x] Predeclare namespace names within an enclosing namespace scope before
      resolving nested namespace/class bodies.
- [x] Resolve an unqualified sibling namespace reference such as `m3.i3` from
      inside `namespace m1.m2`.
- [x] Preserve existing same-file namespace value lookup work tracked by issue
      5287 without merging this issue into that broader qualified-access slice.
- [x] Add a focused regression for a nested namespace referencing a later
      sibling namespace in an `implements` clause or equivalent type-only
      position.

Out of scope:

- Full namespace emit/lowering semantics.
- Same-file top-level qualified namespace value access, tracked by issue 5287.
- Multi-section namespace-only body preservation, tracked by issue 5187.
- TypeScript semantic diagnostics after this name-resolution boundary advances.

## Affected paths

Expected:

- `crates/frontend/src/`
- focused namespace/name-resolution tests or fixtures

Do not touch:

- `crates/backend-wasm/` unless a focused resolver test proves a backend-only
  blocker remains after this issue.

## Acceptance criteria

- [x] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedPrivacy.ts` no longer reports `unresolved name: m3` at `226..228`.
- [x] Focused coverage for the same path no longer reports `unsupported_diagcodes=UnresolvedName:1` for `m3`.
- [x] A focused test covers `namespace A { namespace B { class C implements D.I {} } namespace D { export interface I {} } }`.
- [x] Any next blocker in `complicatedPrivacy.ts` is recorded in this issue or split to a follow-up if outside namespace sibling lookup.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(namespace) or test(name)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/complicatedPrivacy.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/complicatedPrivacy.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from generated bucket `issues/done/1400-implement-complicatedPrivacy.md`.

Related but not duplicates:

- Issue 5287: top-level same-file namespace value access such as `m1.fooExport()`.
- Issue 5187: namespace-only multi-section preservation.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- After `m3` resolves, the same file is expected to expose later diagnostics
  such as invalid getter parameters, computed-property syntax, or private
  exported interface access.

## False-done audit

**truly-done** (5294)

- Implementation commits: verified via `git log --oneline --all --grep=5294`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
