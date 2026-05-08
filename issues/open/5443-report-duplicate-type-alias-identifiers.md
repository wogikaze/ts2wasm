---
id: 5443
title: "Report duplicate type alias identifiers"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TS2300-style duplicate identifier diagnostic when two type aliases
with the same name are present in the same reference compilation view.

## Problem

`namespacesWithTypeAliasOnlyExportsMerge.ts` now build-passes in coverage, but
TypeScript reports the first oracle diagnostic at the duplicate type alias
name `A`:

```ts
// @filename: constAndNS.ts
type A = number;

// @filename: circularWithUses.ts
type A = string;
```

Current compiler evidence:

```text
coverage: build_pass=1
tokens: ok through both type aliases and namespace export lists
ast/resolved: only empty export markers are retained; type aliases are erased before duplicate declaration checks
```

TypeScript oracle evidence:

```text
TS2300: Duplicate identifier 'A'.
TS2300: Duplicate identifier 'A'.
```

Problem: erased type aliases hide duplicate type-only declarations, so the
reference build-passes while TypeScript reports TS2300.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts
```

Current result:

```text
coverage build_pass=1
TypeScript oracle reports TS2300 at duplicate `type A` declarations before later TS2451 and namespace duplicate diagnostics.
```

## Desired final state

The compiler preserves enough type-alias declaration metadata to reject the
representative duplicate `type A` declarations instead of returning a build
pass.

## Scope

In scope:

- [ ] Track type-alias declaration names during type-erasure.
- [ ] Detect duplicate same-scope type-alias names for the representative
  `type A = number; ... type A = string;` shape.
- [ ] Report a source-spanned TS2300-style duplicate identifier diagnostic at
  one or both `A` identifiers.
- [ ] Add focused coverage for duplicate type-alias declarations.

Out of scope:

- Full TypeScript declaration merging.
- TS2451 duplicate block-scoped `declare const try*` diagnostics in
  `namespacesWithTypeAliasOnlyExportsMerge.ts`.
- Duplicate namespace identifiers for `NS1` and `NS2`.
- Runtime lowering for type-only namespace export lists.
- Unknown local binding support for type-only named exports, tracked by
  `issues/open/5438-support-named-exports-of-local-interfaces.md`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/src/`
- focused semantic/parser regression tests

Do not touch:

- backend/runtime ABI
- broad module resolution

## Acceptance criteria

- [ ] `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts` no longer silently build-passes in coverage while TypeScript reports the first TS2300 duplicate `A`.
- [ ] A focused fixture covers duplicate `type A = number; type A = string;`.
- [ ] The diagnostic message or code identifies duplicate identifier `A`.
- [ ] Non-duplicate type aliases still erase/build as before.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend type
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/namespacesWithTypeAliasOnlyExportsMerge.ts --detail --no-dashboard-data
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] not affected

Follow-up issues:

- [ ] none

## Notes

Split from
`issues/done/3435-implement-namespacesWithTypeAliasOnlyExportsMerge.md` on
2026-05-08 after fresh coverage showed the generated import/export blocker was
stale and the first current mismatch is a false build-pass hiding TS2300.

Related but distinct:

- `issues/open/5438-support-named-exports-of-local-interfaces.md` covers
  unknown local binding for named exports of interfaces, not duplicate
  type-alias declarations after a build-pass.

## Completion evidence

Fill when implemented.
