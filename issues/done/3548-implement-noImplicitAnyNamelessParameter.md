---
id: 3548
title: "Implement Noimplicitanynamelessparameter"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed after refreshed evidence split the current parser boundary to
`issues/open/5477-parse-nameless-object-type-method-parameters.md`.

## Problem

Fresh triage shows the parser now consumes several nameless-parameter forms in
ambient variable type annotations, then fails on the object type literal method
signature with multiple nameless parameters:

```text
UnsupportedTypeScriptSyntax: issue-400: unterminated ambient variable declaration type at 173..180
```

Problem: `noImplicitAnyNamelessParameter.ts` has one current parser boundary,
split to issue 5477.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedTypeScriptSyntax:1 unsupported_features=parser-syntax:1
triage: issue-400: unterminated ambient variable declaration type at 173..180
```

## Desired final state

This generated bucket is closed after splitting the current observable parser
behavior into an implementation-ready child issue. Do not implement directly
from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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
- [x] Child issue 5477 contains an exact `reference-triage` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts
```

Not run:

- `cargo fmt --all --check`; issue metadata only.
- `cargo nextest run`; issue metadata only.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] split to `issues/open/5477-parse-nameless-object-type-method-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts`

## Duplicate detection

- `issues/open/5201-parse-object-type-literal-call-signatures.md` covers call
  signatures such as `(name: string): string`, not this method signature with
  nameless primitive/identifier parameters.
- `issues/open/5336-parse-object-type-literal-signatures-with-rest-parameters.md`
  covers a broader rest-parameter mix. The current `d` failure has no rest
  parameter and specifically needs nameless method parameters.
- `issues/open/5245-parse-interface-construct-signatures.md` and
  `issues/open/5257-parse-object-type-literal-construct-signatures.md` cover
  construct signatures, not this method signature.
- Split to issue 5477.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage parser syntax: noImplicitAnyNamelessParameter

- Issue class: triage-needed
- Feature label: parser-syntax
- Diagnostic: UnsupportedTypeScriptSyntax / unsupported-feature-boundary
- Path: reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts
```

Current compiler message:

```text
issue-400: unterminated ambient variable declaration type at 173..180
```

Source context:

```ts
declare var a: { m(...string): void }
declare var b: (string, C) => void;
declare var c: { (C, number): void };
declare var d: { m(boolean, C, object, undefined): void }
```

Visible symbols before failure:

```text
class C
binding a
binding b
binding c
```

Compiler evidence:

```text
tokens: ok through d's method signature tokens
ast/resolved: fail with unterminated ambient variable declaration type
```

TypeScript oracle:

```text
diagnostics: []
d: { m(boolean: any, C: any, object: any, undefined: any): void; }
```

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts --detail --no-dashboard-data
result: pass; reproduced current UnsupportedTypeScriptSyntax parser boundary
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyNamelessParameter.ts
result: pass; generated smart triage evidence and child issue split material
date: 2026-05-08
```

Remaining risks:

- After issue 5477 advances, the same fixture may expose narrower `null` or
  `void` nameless-parameter parsing noted in the source comment.
