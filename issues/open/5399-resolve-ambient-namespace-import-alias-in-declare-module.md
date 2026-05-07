---
id: 5399
title: "Resolve ambient namespace import alias in declare module"
type: feature
area: frontend/name-resolution
class: implementation-ready
priority: P1
depends_on: [5370]
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Resolve an ambient namespace name when it is used as the target of an
import-equals alias inside an ambient external module declaration.

## Problem

Problem: `aliasDoesNotDuplicateSignatures.ts` currently reports
`UnresolvedName: unresolved name: demoNS` for `import alias = demoNS;` inside
`declare module 'demoModule'`.

Fresh triage on 2026-05-08 shows the older ambient namespace syntax boundary is
gone. The parser tokenizes `declare namespace demoNS`, `declare module
'demoModule'`, `import alias = demoNS`, and `export = alias`, but the resolver
does not have a visible ambient namespace binding for `demoNS` in the ambient
module body.

## Current failure

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts
```

Equivalent repo task:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts
```

Representative source:

```ts
declare namespace demoNS {
    function f(): void;
}

declare module 'demoModule' {
    import alias = demoNS;
    export = alias;
}
```

Concrete current failure:

```text
coverage: unsupported=1, unsupported_diagcodes=UnsupportedModule:1, unsupported_features=import-export:1
triage: UnresolvedName unresolved name: `demoNS` at 106..112
resolved dump: later module_graph boundary is issue-232 unsupported non-local module specifier `demoModule`
oracle: TS2664/TS2307 for missing `demoModule`, plus TS2322 for assigning `demoNS.f` to string
```

Compiler evidence:

```text
tokens: ok through declare namespace, declare module, import-equals alias, export assignment, and named import
ast: ok for the user file; ambient declarations are erased before later import/let statements
resolved: current smart-triage stack reports UnresolvedName for `demoNS` in the ambient module import alias
```

## Desired final state

The resolver preserves enough ambient namespace metadata for `import alias =
demoNS` inside `declare module 'demoModule'` to bind the alias target, or it
advances to the already source-spanned module-graph boundary for bare
`demoModule`.

## Scope

In scope:

- [ ] Bind ambient namespace declarations as resolver-visible namespace values
  before resolving ambient external module bodies.
- [ ] Resolve `import alias = demoNS; export = alias;` inside `declare module`
  without reporting `UnresolvedName` for `demoNS`.
- [ ] Preserve ambient erasure: no runtime namespace initialization should be
  emitted for `demoNS`.

Out of scope:

- Base ambient namespace qualified value access, tracked by issue 5370.
- Package or ambient module resolution for bare specifier `demoModule`.
- Declaration emit or signature de-duplication fidelity after module resolution advances.
- Full `export =` runtime/module execution.

## Affected paths

Expected:

- `crates/frontend/src/`
- `crates/ir/src/`
- focused ambient namespace/import-alias resolver tests

Do not touch:

- package resolution or import maps
- backend/runtime code unless this slice exposes a reviewed ambient metadata representation

## Acceptance criteria

- [ ] `aliasDoesNotDuplicateSignatures.ts` no longer reports `UnresolvedName` for `demoNS` in `import alias = demoNS`.
- [ ] A focused fixture covers `declare namespace N { function f(): void; } declare module 'M' { import alias = N; export = alias; }`.
- [ ] The next `aliasDoesNotDuplicateSignatures.ts` blocker, if any, is recorded in this issue or split to a follow-up if outside ambient namespace import-alias resolution.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-ir -E 'test(namespace) or test(resolve) or test(import)'
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/aliasDoesNotDuplicateSignatures.ts --detail --no-dashboard-data
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

Split from generated bucket `579` on 2026-05-08. This issue depends on 5370
because ambient namespace roots must be resolver-visible before ambient module
alias targets can bind reliably.

After `demoNS` resolves, the same reference may expose the bare `demoModule`
module-graph diagnostic already covered by issue 232 behavior, or later
semantic diagnostics for assigning `demoNS.f`/`f` to `string`.

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
