---
id: 5246
title: "Report static declarations inside constructor bodies"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-06
updated: 2026-05-06
---

## Summary

Report a source-spanned parser diagnostic when `static` appears in statement
position inside a constructor/function body.

## Problem

Problem: invalid `static` declarations in constructor bodies report generic unsupported expression instead of a spanned parser diagnostic.

These TypeScript references currently fall through to a generic frontend error:

- `reference/typescript/tests/cases/compiler/class2.ts`
- `reference/typescript/tests/cases/compiler/staticsInConstructorBodies.ts`

Fresh triage reports:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Static, ... })
```

TypeScript instead reports TS1128 at the invalid `static` token.

## Scope

In scope:

- [ ] Detect statement-position `static` before expression parsing reports generic unsupported syntax.
- [ ] Emit a source-spanned invalid statement/declaration diagnostic at `static`.
- [ ] Preserve valid class-member `static` fields and methods.
- [ ] Add focused parser or CLI diagnostic coverage.

Out of scope: runtime semantics and invalid-code AST recovery beyond this
diagnostic.

## Affected paths

Expected: `crates/frontend/src/`, `crates/cli/tests/`, `fixtures/`.

Do not touch: `crates/backend-wasm/` or runtime semantics.

## Acceptance criteria

- [ ] `class2.ts` and `staticsInConstructorBodies.ts` no longer report generic `unsupported expression`.
- [ ] Valid `class C { static f = 3; }` passes.
- [ ] The diagnostic reports the invalid `static` keyword span.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
cargo nextest run -p ts2wasm-cli -E 'test(class) | test(parser)'
```

Impacted commands:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/class2.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/staticsInConstructorBodies.ts
```

## Notes

Split from stale generated buckets `issues/done/1170-implement-class.md` and
`issues/done/4262-implement-staticsInConstructorBodies.md`.
