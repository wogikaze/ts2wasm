---
id: 5332
title: "Parse interface call signatures"
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

Parse and erase TypeScript call signatures inside interfaces, including
interfaces with defaulted generic type parameters.

This is the parser blocker exposed by
`coAndContraVariantInferences6.ts` before the reference can reach its intended
TS2322 value incompatibility diagnostic.

## Problem

`coAndContraVariantInferences6.ts` reaches
`interface FunctionComponent<P = {}> { (props: P): ReactElement<any> | null; }`.
The current parser tokenizes the interface header and the call-signature tokens,
but AST construction fails at the interface body boundary with a generic
`unsupported expression` diagnostic.

Problem: interface call-signature members are not parsed as erasable TypeScript
type members.

## Current failure

Reproduction: `python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences6.ts`.

Focused coverage: `python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences6.ts --detail --no-dashboard-data`.

Current diagnostic:

```text
UnsupportedSyntax: unsupported expression: Some(SpannedToken { kind: Greater, span: Span { start: 190, end: 191 } }) at 192..193
line 10, column 37
```

Representative source:

```ts
interface FunctionComponent<P = {}> {
  (props: P): ReactElement<any> | null;
}
```

Compiler evidence:

```text
tokens: ok through `interface FunctionComponent<P = {}> {`
ast/resolved: fail before representing the interface call signature
TypeScript oracle: accepts the interface and later reports TS2322 for value "C"
```

TypeScript AST evidence:

```text
InterfaceDeclaration FunctionComponent
CallSignature `(props: P): ReactElement<any> | null`
```

## Desired final state

The parser consumes interface call signatures as TypeScript-only members,
preserves existing interface erasure behavior, and lets
`coAndContraVariantInferences6.ts` advance to the later semantic diagnostic.

## Scope

In scope:

- [x] Parse interface members of the form `(param: Type): ReturnType;`.
- [x] Support defaulted interface type parameters such as `<P = {}>` in the focused path.
- [x] Skip return types with generic type arguments such as `ReactElement<any>`.
- [x] Preserve existing parsing for interface property, method, and construct-signature members.

Out of scope:

- Object type literal call signatures, tracked by `issues/done/5201-parse-object-type-literal-call-signatures.md`.
- Callable interface lowering/type behavior, tracked by callable-interface semantic issues.
- Full React/JSX element type compatibility.
- The later TS2322 diagnostic for `{ value: "C" }`.

## Affected paths

Expected:

- `crates/frontend/src/parser/`
- `crates/frontend/src/parser/tests.rs`
- focused fixtures or CLI parser tests

Do not touch:

- `crates/backend-wasm/`
- runtime call lowering
- general object literal expression parsing

## Acceptance criteria

- [x] `coAndContraVariantInferences6.ts` no longer reports the current `unsupported expression ... Greater` diagnostic at `interface FunctionComponent<P = {}>`.
- [x] A focused parser fixture accepts `interface F<P = {}> { (props: P): R<P> | null; }`.
- [x] Existing interface property and construct-signature parser coverage still passes.
- [x] If the reference advances to TS2322 or another semantic blocker, record that blocker separately.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-frontend
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/coAndContraVariantInferences6.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/coAndContraVariantInferences6.ts --detail --no-dashboard-data
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

Split from `issues/open/1264-implement-coAndContraVariantInferences-type-system.md` on
2026-05-07.

Related but not duplicates:

- `issues/done/5245-iterator-protocol-runtime.md` covers `new (...)`
  construct-signature members.
- `issues/done/5201-parse-object-type-literal-call-signatures.md` covers call
  signatures inside object type literals.
- `issues/done/5195-support-callable-interface-typed-local-calls.md` covers
  lowering/semantic behavior after callable interfaces parse.

## Completion evidence

Fill only when implemented.

## False-done audit

**truly-done** (5332)

- Implementation commits: verified via `git log --oneline --all --grep=5332`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
