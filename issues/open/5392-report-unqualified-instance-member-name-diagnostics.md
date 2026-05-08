---
id: 5392
title: "Report unqualified instance member name diagnostics"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Report a source-spanned unresolved-name diagnostic for unqualified instance
member names used from a static method.

## Problem

Problem: `foo` inside a static method currently reports generic
`UnresolvedName` without a stable source span or TS2304-like diagnostic.

Fresh triage on 2026-05-07 shows:

```text
UnresolvedName: unresolved name: `foo`
```

TypeScript reports TS2304 at the same `foo`. It also reports TS2564 for the
instance property initializer, which is outside this resolver slice.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts
```

Fresh evidence:

```text
source: class C { foo: string; static bar() { let k = foo; } }
current: UnresolvedName `foo`
oracle: TS2304 for `foo`; TS2564 strict-property-init is out of scope
```

## Desired final state

The resolver emits a source-spanned TS2304-like diagnostic for unqualified
instance member names used from static methods, without treating the instance
field as an in-scope local or static binding.

## Scope

In scope:

- [ ] Detect unqualified `foo` in a static method when `foo` is an instance member.
- [ ] Emit a source-spanned TS2304-like unresolved-name diagnostic for the method-body `foo`.
- [ ] Re-run the focused reference file and record the next blocker or build-pass result.

Out of scope:

- TS2564 strict-property-initialization diagnostics.
- Static-member suggestion diagnostics, tracked by issue 5391.
- Runtime class field access or property rewriting.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- focused fixtures under the existing resolver/diagnostic test layout

Do not touch:

- backend/runtime code unless this diagnostic slice produces a supported runtime shape
- unrelated scope, namespace, or builtin resolution behavior

## Acceptance criteria

- [ ] `accessStaticMemberFromInstanceMethod01.ts` no longer reports generic `UnresolvedName` without span for `foo`.
- [ ] A focused fixture covers `class C { foo: string; static bar() { let k = foo; } }`.
- [ ] The diagnostic is source-spanned at the method-body `foo` and remains a TS2304-like unresolved-name error.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) or test(member) or test(diagnostic) or test(resolver)'
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/accessStaticMemberFromInstanceMethod01.ts --detail --no-dashboard-data
```

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from:

- `issues/open/564-implement-accessStaticMemberFromInstanceMethod.md`

Related but not duplicate:

- `issues/open/5391-report-unqualified-class-member-name-diagnostics.md`
  owns the mirrored TS2662 static-member suggestion case.

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

- none
