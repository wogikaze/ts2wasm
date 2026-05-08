---
id: 3434
title: "Implement Namespacesdeclaration"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [432]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Closed as superseded by
`issues/open/5409a-report-non-exported-namespace-member-type-annotations.md`.

Fresh focused coverage shows both affected references now build-pass.
`namespacesDeclaration1.ts` also has a clean TypeScript oracle. The remaining
semantic mismatch is `namespacesDeclaration2.ts`, where TypeScript reports
TS2694 for qualified namespace type annotations such as `N.S`, `M.F`, and
`ns.A`. That diagnostic family is already owned by issue 5409.

## Problem

Reference test results show 2 cases fail in directory `namespacesDeclaration` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: namespacesDeclaration had 2 generated reference failures and needed
smart-triage evidence before implementation starts.

Disposition: no child issue created because the current semantic blocker is
covered by existing open issue 5409.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namespacesDeclaration1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namespacesDeclaration1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Close as superseded by an existing implementation-ready owner issue
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Fresh evidence contains exact `reference-triage` commands
- [x] Evidence includes failing paths, diagnostic codes, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Existing owner issue 5409 names the exact current diagnostic family

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 4
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/namespacesDeclaration1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/namespacesDeclaration1.ts
```

Not run:

- `cargo fmt --all --check` (no Rust changes)
- `cargo nextest run` (no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/namespacesDeclaration1.ts`
- `reference/typescript/tests/cases/compiler/namespacesDeclaration2.ts`

## Duplicate detection

- `issues/open/5409a-report-non-exported-namespace-member-type-annotations.md`
  owns TS2694-like diagnostics for qualified namespace type annotations whose
  requested member is not exported or not present.

## Smart triage

Fresh focused coverage:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespacesDeclaration --detail --no-dashboard-data

result:
executed=2
build_pass=2
unsupported=0
blocked=0
semantic_enabled=0

reference/typescript/tests/cases/compiler/namespacesDeclaration2.ts: build_pass
reference/typescript/tests/cases/compiler/namespacesDeclaration1.ts: build_pass
```

Fresh focused triage for `namespacesDeclaration1.ts`:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespacesDeclaration1.ts

result:
BuildPass: ts2wasm build succeeded
TypeScript oracle reports ok with no diagnostics.
```

Source context for `namespacesDeclaration1.ts`:

```ts
namespace M {
   export namespace N {
      export namespace M2 {
         export interface I {}
      }
   }
}
```

Compiler evidence for `namespacesDeclaration1.ts`:

```text
tokens: ok through nested exported namespaces and interface I
ast/resolved: empty retained runtime AST
oracle: ok; no diagnostics
```

Fresh focused triage for `namespacesDeclaration2.ts`:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespacesDeclaration2.ts

result:
BuildPass: ts2wasm build succeeded
TypeScript oracle reports TS2694 for `N.S`, `M.F`, and `ns.A`.
```

Source context for `namespacesDeclaration2.ts`:

```ts
namespace N {
    function S() {}
}
namespace M {
    function F() {}
}

declare namespace ns {
    let f: number;
}

var foge: N.S;
var foo: M.F;
let x: ns.A;
```

Compiler evidence for `namespacesDeclaration2.ts`:

```text
tokens: ok through local namespace functions, ambient namespace ns, and qualified type annotations N.S, M.F, ns.A
ast/resolved: retained runtime statements are foge, foo, and x initialized to undefined
coverage: build_pass=1, unsupported=0
```

TypeScript oracle evidence for `namespacesDeclaration2.ts`:

```text
TS2694: Namespace 'N' has no exported member 'S'.
TS2694: Namespace 'M' has no exported member 'F'.
TS2694: Namespace 'ns' has no exported member 'A'.
```

## Completion evidence

Closed as superseded by issue 5409; no additional child issue created.

Commits:

- `...`

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespacesDeclaration --detail --no-dashboard-data
result: pass; executed=2, build_pass=2
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespacesDeclaration1.ts
result: pass; BuildPass with TypeScript oracle ok/no diagnostics
date: 2026-05-08

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespacesDeclaration2.ts
result: pass; BuildPass with TypeScript oracle TS2694 folded into issue 5409
date: 2026-05-08
```

Remaining risks:

- Issue 5409 must cover both non-exported same-file namespace members and
  absent ambient namespace members in qualified type annotations.
