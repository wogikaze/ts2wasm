---
id: 5391
title: "Report unqualified static member suggestion diagnostics"
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

Report the TypeScript-style TS2662 diagnostic for an unqualified name that
matches a static class member from an instance method.

## Problem

Problem: `foo` inside an instance method currently reports generic
`UnresolvedName` without the TypeScript `C.foo` static-member suggestion.

Fresh triage on 2026-05-07 shows the focused reference file still stops at:

```text
UnresolvedName: unresolved name: `foo`
```

TypeScript reports a sharper diagnostic for the static member case:

```text
Cannot find name 'foo'. Did you mean the static member 'C.foo'?
```

The mirrored instance-member/static-method file is tracked separately by issue
5392.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts
```

Fresh evidence:

```text
accessInstanceMemberFromStaticMethod01.ts:
  source: class C { static foo: string; bar() { let k = foo; } }
  current: UnresolvedName `foo`
  oracle: TS2662, did you mean static member `C.foo`

```

## Desired final state

The resolver preserves enough class static-member context to classify the
unqualified miss and report a source-spanned diagnostic that matches the
TypeScript oracle for this focused case.

## Scope

In scope:

- [ ] Detect unqualified `foo` in an instance method when `C.foo` is a static member.
- [ ] Emit a source-spanned diagnostic equivalent to TS2662 for the static-member suggestion.
- [ ] Re-run the focused reference file and record the next blocker or build-pass result.

Out of scope:

- Strict-property-initialization diagnostics such as TS2564.
- Rewriting unqualified member access into runtime property access.
- Static-method access to an instance member, tracked by issue 5392.
- Broad name-resolution buckets unrelated to class member lookup suggestions.

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

- [ ] `accessInstanceMemberFromStaticMethod01.ts` no longer reports generic `UnresolvedName` for `foo`.
- [ ] A focused fixture covers `class C { static foo: string; bar() { let k = foo; } }`.
- [ ] The diagnostic for the static-member case is source-spanned at `foo` and names `C.foo`.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(class) or test(member) or test(diagnostic) or test(resolver)'
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts --detail --no-dashboard-data
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

- `issues/open/562-implement-accessInstanceMemberFromStaticMethod.md`

Related but not duplicate:

- `issues/open/437-implement-name-resolution.md` is a broad generated name-resolution bucket.
- `issues/open/5392-report-unqualified-instance-member-name-diagnostics.md`
  owns the mirrored static-method/instance-member TS2304 case.

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
