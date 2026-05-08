---
id: 1271
title: "Implement Collisionargumentsintype"
type: spike
area: frontend/syntax
class: blocked
priority: P1
depends_on: [5000]
blocks: []
created: 2026-05-01
updated: 2026-05-07
---
> **Reopened by false-done audit** (2026-05-07)
> Classification: must-reopen
> Reason: Batch-migrated to issues/open/ without implementation commits.
> Evidence: Empty completion evidence. No feat/fix commit for #1271.

## Summary

Triage collisionArgumentsInType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `collisionArgumentsInType` with diagnostics: parser-syntax. Fresh triage confirms the current blocker is an object type literal parser boundary, now split into issue 5336.

Problem: collisionArgumentsInType has 1 reference failure and needed smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionArgumentsInType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsInType.ts --detail
```

## Desired final state

This generated bucket is closed. Implementation proceeds through issue 5336 for the object type literal signature parser blocker.

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
- [x] Child issue 5336 contains an exact reference-triage command
- [x] Child issue 5336 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript evidence
- [x] Child issue 5336 acceptance names the exact reference path and diagnostic change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/collisionArgumentsInType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/collisionArgumentsInType.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5336-parse-object-type-literal-signatures-with-rest-parameters.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionArgumentsInType.ts`

## Duplicate detection

- `issues/open/5201-parse-object-type-literal-call-signatures.md` covers object type literal call-signature members only.
- `issues/open/5257-parse-object-type-literal-construct-signatures.md` covers construct signatures only.
- `issues/open/5333-report-strict-mode-arguments-binding-diagnostics.md` covers strict-mode `arguments` diagnostics after syntax parsing succeeds.
- none fully cover the mixed call, construct, method, and property signature members with rest parameters in this reference.

## Smart triage

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

Representative source:

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

## Completion evidence

Fill only when moving to `done/`.

Commits:

- split to `issues/open/5336-parse-object-type-literal-signatures-with-rest-parameters.md`

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionArgumentsInType.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; current blocker split to issue 5336
date: 2026-05-07
```

Remaining risks:

- After issue 5336 lands, this reference will likely expose TS1100 strict-mode
  `arguments` diagnostics, related to issue 5333 but involving TypeScript
  signature parameter names rather than runtime bindings.
