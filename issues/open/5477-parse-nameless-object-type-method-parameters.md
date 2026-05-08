---
id: 5477
title: "Parse nameless object type method parameters"
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

Parse and erase object type literal method signatures whose parameters omit
names, such as `m(boolean, C, object, undefined): void` in an ambient variable
type annotation.

## Problem

`noImplicitAnyNamelessParameter.ts` tokenizes the declaration but AST
construction reports:

```text
UnsupportedTypeScriptSyntax: issue-400: unterminated ambient variable declaration type at 173..180
```

Problem: object type literal method signatures with nameless parameters are not
consumed as complete TypeScript type annotations.

## Current failure

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts
```

Current failing source:

```ts
declare var d: { m(boolean, C, object, undefined): void }
```

Compiler evidence:

```text
tokens: ok through `m(boolean, C, object, undefined): void`
ast/resolved: fail with issue-400 unterminated ambient variable declaration type
TypeScript oracle: diagnostics=[], d has method parameters boolean/C/object/undefined typed as any
```

## Desired final state

The parser consumes the focused object type literal method signature and erases
it consistently with existing ambient type annotation handling.

## Scope

In scope:

- [ ] Parse object type literal method-signature parameters without explicit names.
- [ ] Cover primitive/identifier spellings from the representative: `boolean`, `C`, `object`, `undefined`.
- [ ] Re-run the representative triage and record any next parser boundary separately.

Out of scope:

- Nameless `null` or `void` parameter forms mentioned by the source comment.
- Object type literal call signatures, construct signatures, or rest-parameter mixes tracked by existing issues.
- Type checking noImplicitAny diagnostics for nameless parameters.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- focused parser or CLI fixture

Do not touch:

- `crates/backend-wasm/`
- resolver/lowering call semantics

## Acceptance criteria

- [ ] `noImplicitAnyNamelessParameter.ts` no longer reports `issue-400: unterminated ambient variable declaration type` at `173..180`.
- [ ] A focused parser fixture covers `declare var d: { m(boolean, C, object, undefined): void }`.
- [ ] Existing object type literal call-signature and construct-signature parser fixtures still pass.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(parser) or test(type)'
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts --detail --no-dashboard-data
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

Split from `issues/done/3548-implement-noImplicitAnyNamelessParameter.md`.

Related but not duplicates:

- `issues/open/5201-parse-object-type-literal-call-signatures.md`
- `issues/open/5336-parse-object-type-literal-signatures-with-rest-parameters.md`
- `issues/open/5257-parse-object-type-literal-construct-signatures.md`

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
