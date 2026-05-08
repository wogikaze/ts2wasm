---
id: 1223
title: "Implement Classmemberinitializerwithlamdascoping Import Export"
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
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/done/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1223.

## Summary

Triage classMemberInitializerWithLamdaScoping-import-export across 2 failing
reference test cases and split/supersede the stale generated bucket with current
smart-triage evidence.

## Problem

Reference test results previously showed 2 cases failing in directory
`classMemberInitializerWithLamdaScoping-import-export` with diagnostics:
import-export. Fresh triage shows the current blockers are narrower module
export boundaries.

Problem: the generated bucket mixed two different first blockers:
`classMemberInitializerWithLamdaScoping4.ts` stops at an existing entry
`export var` boundary, while `classMemberInitializerWithLamdaScoping3.ts` stops
at a dependency-module `export class` boundary that needed a new focused owner.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping4.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping4.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm that `classMemberInitializerWithLamdaScoping4.ts` is covered by existing issue 5283
- [x] Split `classMemberInitializerWithLamdaScoping3.ts` into focused child issue 5324
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

- [x] Duplicate candidates below are confirmed as no-match, superseded, or split
- [x] Child issue contains an exact `python scripts/manager.py reference-triage ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping4.ts --detail --no-dashboard-data
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping4.ts
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts
```

Not run:

- `cargo fmt --all --check`; issue split only, no Rust code changed
- `cargo nextest run`; issue split only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/done/5324-support-dependency-export-class-declarations.md`
- [x] superseded in part by `issues/done/5283-support-entry-export-var-declarations.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping4.ts`
- `reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts`

## Duplicate detection

- `issues/open/432-implement-import-export.md` - Implement import/export module syntax (same feature label, title overlap)
- `issues/done/457-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/open/463-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/543-implement-APISample-import-export.md` - Implement Apisample Import Export (same feature label, title overlap)
- `issues/done/549-implement-FunctionDeclaration-import-export.md` - Implement Functiondeclaration Import Export (same feature label, title overlap)
- `issues/open/662-implement-arrayAssignmentTest-import-export.md` - Implement Arrayassignmenttest Import Export (same feature label, title overlap)
- `issues/open/732-implement-assignmentCompatability-import-export.md` - Implement Assignmentcompatability Import Export (same feature label, title overlap)
- `issues/open/766-implement-augmentedTypesEnum-import-export.md` - Implement Augmentedtypesenum Import Export (same feature label, title overlap)
- `issues/done/055-implement-import-export.md` - Umbrella: implement import and export (same feature label, title overlap)
- `issues/done/5283-support-entry-export-var-declarations.md` - exact owner for the `classMemberInitializerWithLamdaScoping4.ts` first blocker, `export var field1: string`
- `issues/done/5232-w0-fixture-ize-runtimelinkplan-linker-structure-tests.md` - related entry-module export-class owner, not exact for dependency virtual files
- `issues/done/5295-resolve-import-equals-require-to-virtual-node-modules-class-export.md` - related node_modules dependency export-class shape, not exact for this fixed reference window

## Smart triage

Fresh triage shows this generated import/export bucket currently contains two
separate first blockers.

### Smart triage: classMemberInitializerWithLamdaScoping4

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-055: unsupported variable export; module resolution and loading are not implemented`
- Path: `reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping4.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping4.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping4.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-system-amd:1
```

Source context:

```ts
// @module: commonjs
// @Filename: classMemberInitializerWithLamdaScoping3_0.ts
export var field1: string;
```

Compiler evidence:

```text
tokens: ok; Export, Var, Ident("field1"), Colon, Ident("string")
ast/resolved: fail at issue-055 unsupported variable export
```

Disposition:

- Superseded by `issues/done/5283-support-entry-export-var-declarations.md`.

### Smart triage: classMemberInitializerWithLamdaScoping3

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Current compiler message: `issue-5005: dependency module declaration export uses a form outside the current static export slice`
- Path: `reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts --detail --no-dashboard-data
```

Coverage result:

```text
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=module-system-amd:1
```

Source context:

```ts
// @Filename: classMemberInitializerWithLamdaScoping3_0.ts
var field1: string;

// @Filename: classMemberInitializerWithLamdaScoping3_1.ts
declare var console: {
    log(msg?: any): void;
};
export class Test1 {
    constructor(private field1: string) {
    }
    messageHandler = () => {
        console.log(field1);
    };
}
```

Compiler evidence:

```text
tokens: ok
ast: ok; entry var plus dependency ExportDecl(ClassDecl Test1)
resolved: ok; constructor parameter property is represented
module build: UnsupportedModule issue-5005 for dependency-module export class
```

TypeScript oracle evidence:

```text
typescript ok: false
diagnostic TS2301: Initializer of instance member variable 'messageHandler' cannot reference identifier 'field1' declared in the constructor.
```

Split result:

- `issues/done/5324-support-dependency-export-class-declarations.md`

## Completion evidence

Commits:

- Split to `issues/done/5324-support-dependency-export-class-declarations.md`; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping4.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; issue-055 unsupported variable export, superseded by issue 5283
date: 2026-05-07

command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classMemberInitializerWithLamdaScoping3.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; issue-5005 dependency-module export class, split to issue 5324
date: 2026-05-07
```

Remaining risks:

- After issue 5283 and issue 5324 land, these files may expose the later TS2301
  class field initializer scoping diagnostic. That semantic parity is outside
  this generated bucket cleanup.
