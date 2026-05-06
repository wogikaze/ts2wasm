---
id: 1232
title: "Implement Classpropinitializationinferencewithelementaccess"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
status: done
---

## Summary

Triage classPropInitializationInferenceWithElementAccess across 1 failing
reference test case and close it as superseded by the existing entry export
class issue.

## Problem

Reference test results previously showed 1 case failing in directory
`classPropInitializationInferenceWithElementAccess` with diagnostics:
import-export. Fresh triage shows the class body parses and resolves; the
current first blocker is the entry-module `export class` boundary.

Problem: `export class Cls { ... }` stops with issue-5005 before later
class-property initialization inference parity can be evaluated.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classPropInitializationInferenceWithElementAccess.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classPropInitializationInferenceWithElementAccess.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm existing issue 5232 covers the current first blocker
- [x] Supersede this generated bucket without creating a duplicate child
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

- [x] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [x] Superseding issue 5232 contains the implementation-ready entry export class boundary
- [x] This issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classPropInitializationInferenceWithElementAccess.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classPropInitializationInferenceWithElementAccess.ts
```

Not run:

- `cargo fmt --all --check`; issue close only, no Rust code changed
- `cargo nextest run`; issue close only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] superseded by `issues/open/5232-support-entry-export-class-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classPropInitializationInferenceWithElementAccess.ts`

## Duplicate detection

- `issues/open/5232-support-entry-export-class-declarations.md` - exact current first blocker for entry-module `export class`
- `issues/open/5255-resolve-super-property-accesses.md` and element-access issues are not current blockers; this case resolves `this['x']`, `this['y']`, `this['z']`, and `this[0]` before module build fails

## Smart triage

Fresh triage shows this generated import/export bucket is currently blocked by
entry-module export class support.

### Smart triage: classPropInitializationInferenceWithElementAccess

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-5005: entry module export Cls uses a declaration form outside the current static export slice`
- Path: `reference/typescript/tests/cases/compiler/classPropInitializationInferenceWithElementAccess.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classPropInitializationInferenceWithElementAccess.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classPropInitializationInferenceWithElementAccess.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=type-system:1
```

Source context:

```ts
export class Cls {
    x;
    y;
    z;

    0;

    constructor(seed: number) {
        this['x'] = [seed];
        this['y'] = { seed };
        this['z'] = `${seed}`;
        this[0] = [seed];
    }
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; ExportDecl(ClassDecl Cls)
resolved: ok; string element assignments become PropertyAssign x/y/z and numeric element assignment becomes PropertyAssignDynamic
module build: UnsupportedModule issue-5005 for entry-module export class
```

TypeScript oracle evidence:

```text
typescript ok: true
diagnostics: []
AST topLevel includes exported ClassDeclaration Cls.
```

Superseding owner:

- `issues/open/5232-support-entry-export-class-declarations.md`

## Completion evidence

Commits:

- Superseded by `issues/open/5232-support-entry-export-class-declarations.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classPropInitializationInferenceWithElementAccess.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; entry export class issue-5005 superseded by issue 5232
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classPropInitializationInferenceWithElementAccess.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; unsupported=1
date: 2026-05-07
```

Remaining risks:

- After issue 5232 lands, this reference may expose later class property
  inference or semantic parity work around element access initialization.
