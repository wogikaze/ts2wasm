---
id: 1508
title: "Implement Contextualtypebasedonintersectionwithanyinthemix Unknown Unsupported"
type: spike
area: frontend/syntax
class: done
priority: P2
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
---

## Summary

Triage contextualTypeBasedOnIntersectionWithAnyInTheMix-unknown-unsupported across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `contextualTypeBasedOnIntersectionWithAnyInTheMix-unknown-unsupported` with diagnostics: unknown-unsupported. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: contextualTypeBasedOnIntersectionWithAnyInTheMix-unknown-unsupported has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix1.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix1.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Fix the exact parser unsupported boundary for interface generic defaults
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

- [x] Duplicate candidates below are confirmed; the residual blocker is folded into issue 5161
- [x] Focused parser coverage covers the exact interface generic default construct
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix1.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix1.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] folded residual name-resolution blocker into `issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix1.ts`

## Duplicate detection

- `issues/open/694-implement-arrayToLocaleStringES-unknown-unsupported.md` - Implement Arraytolocalestringes Unknown Unsupported (same feature label, title overlap)

## Smart triage

Fresh triage on 2026-05-07 showed the generated `unknown-unsupported` bucket
was a parser-erasure bug in interface generic parameter lists:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Comma, span: Span { start: 288, end: 289 } }) at 292..293
```

The parser consumed the first `{` in `O extends object = {}` as the interface
body because `consume_typescript_interface_declaration` skipped from the
interface name to the first left brace without first consuming the generic
parameter list. The fix reuses the existing TypeScript generic parameter list
skipper before searching for the interface body.

After the fix, focused coverage advances this path out of unknown-unsupported
and into the existing ambient value name-resolution boundary:

```text
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix1.ts: UnresolvedName: name-resolution
```

Current residual diagnostic:

```text
UnresolvedName: unresolved name: `styled` at 806..812
```

Source context:

```ts
declare const styled: StyledInterface;
declare const Flex: (props: BaseProps) => null;

export const StyledSelect = styled(Flex).attrs({
  as: "select",
});
```

The residual `declare const styled` expression reference is covered by
`issues/open/5161-model-ambient-value-declarations-for-name-resolution.md`.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- pending

Validation result:

```text
command: cargo test -p ts2wasm-frontend parses_typescript_interface_generic_defaults_as_erased_syntax
result: pass
date: 2026-05-07

command: cargo fmt --all --check
result: pass
date: 2026-05-07

command: cargo nextest run -p ts2wasm-frontend
result: pass; 172 passed
date: 2026-05-07

command: cargo build -p ts2wasm-cli
result: pass
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix1.ts --detail --no-dashboard-data
result: pass; unknown-unsupported cleared, current result is UnresolvedName/name-resolution
date: 2026-05-07

command: env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/contextualTypeBasedOnIntersectionWithAnyInTheMix1.ts
result: pass; current residual is UnresolvedName for ambient const `styled`
date: 2026-05-07
```

Remaining risks:

- The file still does not build-pass because ambient value declarations are erased before name resolution; that broader resolver work is owned by issue 5161.
