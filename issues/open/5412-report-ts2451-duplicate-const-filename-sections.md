---
id: 5412
title: "Report TS2451 for duplicate const filename sections"
type: feature
area: frontend/semantics
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Report a TypeScript-style TS2451 diagnostic for duplicate block-scoped `const`
names in the representative `moduleDetectionIsolatedModulesCjsFileScope.ts`
reference case.

## Problem

The reference case has two virtual sections that each declare `const a = 2;`:

```ts
// @filename: filename.cts
const a = 2;
// @filename: filename.mts
const a = 2;
```

The compiler already detects the duplicate as `DuplicateLocal`, but it remains
an unsupported blocker instead of a TypeScript-style diagnostic. TypeScript
reports TS2451 at both `a` identifiers.

Problem: duplicate `const a` across the representative `.cts`/`.mts` reference
sections reports generic `DuplicateLocal` instead of TS2451-style diagnostics.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleDetectionIsolatedModulesCjsFileScope --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=DuplicateLocal:1
unsupported_features=duplicate-local:1
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleDetectionIsolatedModulesCjsFileScope.ts
```

Compiler evidence:

```text
tokens: ok for both @filename sections and both const a declarations
ast: two Let { name: "a", is_var: false } declarations
resolved: DuplicateLocal duplicate local binding: `a` at 191..203
```

TypeScript oracle:

```text
TS2451: Cannot redeclare block-scoped variable 'a'. at line 7, character 7
TS2451: Cannot redeclare block-scoped variable 'a'. at line 9, character 7
```

## Desired final state

The compiler reports a source-spanned TS2451-style duplicate block-scoped
variable diagnostic for the representative duplicate `const a` case instead of
leaving it as an unsupported `DuplicateLocal` blocker.

## Scope

In scope:

- [ ] Map this true duplicate block-scoped binding to a TS2451-style diagnostic.
- [ ] Span the diagnostic at the duplicate identifier, not only the whole `const`
      declaration.
- [ ] Add one focused regression for duplicate `const a` in the same effective
      script scope.
- [ ] Preserve existing valid `var` redeclaration tolerance.

Out of scope:

- Exported external-module binding isolation, tracked by issue 5368.
- Broad test262 duplicate-local reduction, tracked by issue 343.
- All duplicate identifier diagnostic variants.

## Affected paths

Expected:

- `crates/ir/src/name_resolver.rs`
- focused frontend/IR tests or fixtures

Do not touch:

- backend/runtime lowering
- package or on-disk module resolution

## Acceptance criteria

- [ ] `moduleDetectionIsolatedModulesCjsFileScope.ts` no longer reports generic `DuplicateLocal`; it reports TS2451-style `Cannot redeclare block-scoped variable 'a'.`.
- [ ] A focused regression covers duplicate `const a` declarations and asserts the duplicate identifier span.
- [ ] Compatible `var` redeclarations continue to build.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(duplicate) or test(scope)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleDetectionIsolatedModulesCjsFileScope.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleDetectionIsolatedModulesCjsFileScope --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
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

Split from generated bucket
`issues/done/3333-implement-moduleDetectionIsolatedModulesCjsFileScope.md`.

Related but not duplicates:

- `issues/open/5368-isolate-exported-bindings-across-filename-sections.md`
  covers false `DuplicateLocal` for duplicate exported names in different
  external-module sections.
- `issues/open/343-implement-duplicate-local-detection.md` is the blocked broad
  test262 parent.
- `issues/open/5307-report-var-function-duplicate-identifier-diagnostics.md`
  covers var/function collisions, not duplicate block-scoped `const`.

## Completion evidence

Fill when implemented.
