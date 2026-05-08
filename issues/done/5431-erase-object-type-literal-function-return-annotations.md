---
id: 5431
title: "Erase object type literal function return annotations"
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

Parse and erase plain object type literal function return annotations so annotation members do not become runtime statements.

## Problem

`reference/typescript/tests/cases/compiler/multiLineErrors.ts` fails before the expected semantic diagnostics because the parser treats the return annotation in `function noReturn(): { n: string; y: number; }` as the function body.

Current reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiLineErrors.ts
```

Current result:

```text
UnresolvedName: unresolved name: string at 66..72
```

Focused coverage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiLineErrors.ts --detail --no-dashboard-data
```

Current coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnresolvedName:1
unsupported_features=name-resolution:1
```

## Source Context

```ts
function noReturn(): {
    n: string;
    y: number;
}
{
    var x = 4;
    var y = 10;
}
```

The TypeScript parser records the return annotation as a `TypeLiteral`. The current frontend AST consumes the annotation braces as the function body, emits labeled statements for `n: string;` and `y: number;`, and leaves the real body block as a top-level block.

## Desired final state

The frontend recognizes and erases plain object type literal return annotations on functions. The real function body must start at the post-annotation `{ ... }`, and identifiers inside annotation property signatures must not be resolved as runtime names.

## Scope

In scope:

- [ ] Parse and erase return annotations of the form `function f(): { p: string; q: number; } { ... }`
- [ ] Skip semicolon-delimited property signatures inside a plain object type literal return annotation
- [ ] Preserve the actual function body after the return annotation
- [ ] Add a focused parser or AST regression covering this syntax
- [ ] Re-run focused triage for `multiLineErrors.ts`

Out of scope:

- Type predicate object return annotations such as `x is { a: string; }`; see #5235
- Construct signatures inside object type literals such as `{ new(): Object }`; see #5257
- Emitting TypeScript semantic diagnostics TS2355 and TS2322
- Runtime or backend changes

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`

Do not touch:

- `crates/runtime-abi/`
- `crates/backend-wasm/`
- unrelated reference harness scripts

## Acceptance criteria

- [ ] The parser no longer turns `n: string;` and `y: number;` annotation members into labeled function body statements
- [ ] The real `{ var x = 4; var y = 10; }` block remains the function body
- [ ] A focused regression test covers a function returning `{ n: string; y: number; }`
- [ ] Focused triage for `multiLineErrors.ts` advances past the current `UnresolvedName: string` blocker or reaches a documented semantic diagnostic blocker

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multiLineErrors.ts
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multiLineErrors.ts --detail --no-dashboard-data
```

Not run:

- none

## Notes

This issue was split from #3404 after focused triage on 2026-05-08. Existing issues #5235 and #5257 cover adjacent but narrower object-return annotation forms.
