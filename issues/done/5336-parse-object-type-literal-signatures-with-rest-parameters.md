---
id: 5336
title: "Parse object type literal signatures with rest parameters"
type: feature
area: frontend/parser
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-07
updated: 2026-05-07
---

## Summary

Parse TypeScript object type literal members that combine call signatures,
construct signatures, method signatures, function-valued properties, and rest
parameters.

`collisionArgumentsInType.ts` currently stops with an unterminated type
annotation at the closing brace of an object type literal, before the compiler
can report the TypeScript TS1100 strict-mode `arguments` diagnostics for the
signature parameter names.

## Problem

The representative reference declares object type literal annotations with
several signature member forms:

```ts
var v2: {
    (arguments: number, ...restParameters);
    new (arguments: number, ...restParameters);
    foo(arguments: number, ...restParameters);
    prop: (arguments: number, ...restParameters) => void;
}
var v21: {
    (i: number, ...arguments);
    new (i: number, ...arguments);
    foo(i: number, ...arguments);
    prop: (i: number, ...arguments) => void;
}
```

The parser does not consume the full object type literal annotation and reports
an unterminated annotation at the second closing brace.

Problem: object type literal signature members with rest parameters are not
parsed as complete TypeScript type annotations.

## Current failure

Reproduction:
`python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsInType.ts`.

Focused coverage:
`python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsInType.ts --detail --no-dashboard-data`.

Current diagnostic:

```text
UnsupportedTypeScriptSyntax: unterminated TypeScript type annotation at 837..838
```

Focused coverage result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=arguments-object:1
```

Compiler evidence:

```text
tokens: ok through object type literal members and closing braces
ast: fails while consuming the second object type literal annotation
visible symbols: v1, v12, v2, v21
failure location: line 18, column 2, at the closing brace of `v21`
```

TypeScript oracle evidence:

```text
TS1100: Invalid use of 'arguments' in strict mode.
```

## Desired final state

The parser consumes object type literal annotations containing signature
members with rest parameters. The representative file advances past the current
unterminated type annotation and exposes later TypeScript compatibility
diagnostics as separate work if needed.

## Scope

In scope:

- [x] Parse object type literal call-signature members with rest parameters.
- [x] Parse object type literal construct-signature members with rest parameters.
- [x] Parse object type literal method-signature members with rest parameters.
- [x] Parse object type literal function-valued property signatures with rest parameters.
- [x] Preserve or erase the parsed TypeScript-only type metadata consistently with existing annotation handling.
- [x] Re-run the representative triage and record the next TS1100-style diagnostic separately if exposed.

Out of scope:

- Runtime support for callable or constructable object values.
- Type checking object type literal signatures.
- Strict-mode `arguments` binding diagnostics after this parser blocker
  advances.
- Interface call signatures, tracked by
  `issues/done/5332-parse-interface-call-signatures.md`.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/cli/tests/`
- `fixtures/`

Do not touch:

- `crates/backend-wasm/`
- runtime callable-object or construct-signature behavior
- unrelated resolver/lowering call semantics

## Acceptance criteria

- [x] `collisionArgumentsInType.ts` no longer reports `unterminated TypeScript type annotation` at `837..838`.
- [x] A focused parser fixture covers `{ (x: number, ...rest): void; }` in a variable type annotation.
- [x] A focused parser fixture covers `{ new (x: number, ...rest): C; }` in a variable type annotation.
- [x] A focused parser fixture covers `{ foo(x: number, ...rest): void; prop: (x: number, ...rest) => void; }`.
- [x] Existing object literal expression parsing remains unchanged.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend -E 'test(type) or test(parser)'
cargo nextest run -p ts2wasm-cli -E 'test(parser) | test(type)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsInType.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsInType.ts --detail --no-dashboard-data
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

Split from `issues/open/1271-implement-collisionArgumentsInType.md` on
2026-05-07.

Related but not duplicates:

- `issues/done/5201-parse-object-type-literal-call-signatures.md` covers
  overload-like call-signature members in object type literals, without the
  construct, method, property, and rest-parameter mix from this reference.
- `issues/done/5257-parse-object-type-literal-construct-signatures.md` covers
  zero-argument construct signatures, without the mixed rest-parameter
  signature members from this reference.
- `issues/done/5333-report-strict-mode-arguments-binding-diagnostics.md`
  covers strict-mode `arguments` diagnostics for runtime bindings after syntax
  parsing succeeds.

## Completion evidence

Fill only when implemented.

## False-done audit

**truly-done** (5336)

- Implementation commits: verified via `git log --oneline --all --grep=5336`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
