---
id: 5423
title: "Parse declaration-file exported const declarations"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Accept ambient `export const NAME: Type;` declarations in virtual declaration
file sections so module-resolution reference cases can advance past `.d.ts`
surface declarations.

## Problem

`moduleResolutionAsTypeReferenceDirective.ts` contains a virtual declaration
file section:

```ts
// @Filename: /typings/phaser/types/phaser.d.ts
export const a2: number;
```

The current frontend parses this like an executable `const` declaration and
fails before module/type-reference resolution is reachable:

```text
UnsupportedSyntax: const declarations require an initializer at 233..235
```

In a `.d.ts` file, the declaration is ambient surface metadata. It should parse
without a runtime initializer and should not be lowered as executable code.

## Current failure

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirective.ts
```

Observed 2026-05-08:

```text
diagnosis: UnsupportedSyntax / parser-or-frontend-unsupported
message: const declarations require an initializer at 13..15
actual error span: 233..235
source:
// @Filename: /typings/phaser/types/phaser.d.ts
export const a2: number;

// @Filename: /typings/phaser/package.json
{ "name": "phaser", "version": "1.2.3", "types": "types/phaser.d.ts" }

// @Filename: /a.ts
import { a2 } from "phaser";
```

The token stream includes `Export Const Ident("a2") : Ident("number") ;`,
then the virtual `package.json` section, then `import { a2 } from "phaser";`.

## Desired final state

Declaration-file exported const declarations parse as ambient exports, preserve
enough export metadata for later resolver/module-resolution work, and do not
emit runtime initializer code.

## Scope

In scope:

- [ ] Detect declaration-file virtual sections such as `.d.ts`.
- [ ] Accept `export const NAME: Type;` without an initializer in declaration files.
- [ ] Preserve source spans and symbol/export metadata needed by the resolver.
- [ ] Add focused parser or compiler regression coverage for a virtual `.d.ts`
      section with `export const a: number;`.

Out of scope:

- Missing initializer diagnostics for executable `.ts` const declarations; issue `5350`.
- Typed const declarations with valid initializers; issue `5264`.
- Skipping virtual `package.json` sections; issue `5402`.
- Type reference directive resolution itself; issue `227`.
- Bare package/module resolution for `phaser`.
- Declaration emit or full `.d.ts` type checking.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/ast.rs`
- `crates/cli/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- runtime/backend code unless lowering proves a declaration-file node reaches codegen

## Acceptance criteria

- [ ] `reference-triage` for
      `reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirective.ts`
      no longer reports `const declarations require an initializer` at
      `export const a2: number;`.
- [ ] A focused regression covers:
      `// @Filename: /pkg/index.d.ts` followed by `export const a: number;`.
- [ ] Ordinary executable `.ts` code such as `const x: number;` still reports
      a missing-initializer diagnostic or remains owned by issue `5350`; it is
      not silently accepted as executable code.
- [ ] If the next blocker becomes the virtual `package.json` section or bare
      `phaser` package resolution, the triage output points to issue `5402` or
      module-resolution owners instead of this issue.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend const
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm \
  python scripts/manager.py reference-triage tsc \
  reference/typescript/tests/cases/compiler/moduleResolutionAsTypeReferenceDirective.ts
```

Impacted commands:

```sh
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

## Notes

Split from `issues/done/3373-implement-moduleResolutionAsTypeReferenceDirective.md`.
Issue `5350` was checked and is intentionally kept as the executable `.ts`
negative-diagnostic owner, not the declaration-file acceptance owner.

## Completion evidence

Fill only when moving to `done/`.
