---
id: 5205
title: "Report incompatible var redeclaration type diagnostics"
type: feature
area: frontend/resolver
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Report a TypeScript-compatible diagnostic when repeated `var` declarations in
the same var scope have incompatible inferred types.

## Problem

`capturedLetConstInLoop14.ts` now tokenizes, parses, resolves, and builds
successfully, but TypeScript reports TS2403 for `var v;` after an earlier
`var v = 1;`. The generated duplicate-local bucket is no longer blocked by a
compiler failure, but the missing diagnostic still represents an observable
semantic gap.

Problem: incompatible same-scope `var` redeclarations can build successfully instead of reporting a TS2403-style diagnostic.

## Current failure

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop14.ts
```

Current compiler result:

```text
ts2wasm build succeeded
```

TypeScript oracle diagnostic:

```text
TS2403: Subsequent variable declarations must have the same type.
Variable 'v' must be of type 'number', but here has type 'any'.
```

Representative source:

```ts
function foo(x: number) {
  var v = 1;
  do {
    let x = v;
    var v;
    var v = 2;
    (() => x + v);
  } while (false);
  use(v);
}
```

Triage evidence:

- Tokens, AST, and resolved dumps succeed.
- AST contains outer `var v = 1`, inner `let x = v`, bodyless `var v`, and
  `var v = 2` inside a `do` block.
- TypeScript oracle reports TS2403 at the bodyless `var v` declaration.

## Desired final state

The resolver or type-checking layer records same-var-scope redeclarations and
reports a source-spanned diagnostic when a later `var` declaration's inferred
type is incompatible with the earlier one. Compatible `var` redeclarations
remain governed by issue 5162.

## Scope

In scope:

- [ ] Track repeated `var` declarations in the same var/function scope
- [ ] Compare the representative inferred type shape for initialized and
  uninitialized `var` declarations
- [ ] Report a source-spanned TS2403-style diagnostic at the later declaration
- [ ] Preserve accepted compatible `var` redeclarations

Out of scope:

- Full TypeScript type compatibility across arbitrary declaration merging
- Ambient/lib redeclarations, tracked by `issues/open/5176-report-ambient-var-lib-redeclaration-diagnostics.md`
- Duplicate `let` / `const` behavior

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- unrelated loop lowering/runtime semantics

## Acceptance criteria

- [ ] `capturedLetConstInLoop14.ts` no longer silently build-passes when
  TypeScript reports TS2403 for `var v`
- [ ] A focused fixture covers `var v = 1; do { var v; var v = 2; } while
  (false);`
- [ ] The diagnostic is source-spanned at the later `var v` declaration
- [ ] Compatible same-scope `var` redeclarations from issue 5162 remain allowed

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli -E 'test(var) | test(resolution)'
```

Impacted commands:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/capturedLetConstInLoop14.ts
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

Related issue 5162 removes false `DuplicateLocal` blockers for compatible
`var` redeclarations. This issue handles the next diagnostic step once the
compiler already accepts the declarations.

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
