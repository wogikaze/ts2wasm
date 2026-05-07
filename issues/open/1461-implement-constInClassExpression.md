---
id: 1461
title: "Implement Constinclassexpression"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1461.

## Summary

Closed after splitting the current first blocker into
`issues/open/5354-report-invalid-const-class-members.md`. Fresh triage shows
`constInClassExpression.ts` now builds, but TypeScript rejects the invalid
`const a = 4;` class member with TS1248.

## Problem

The generated bucket originally represented a parser-syntax failure. Current
coverage no longer shows an unsupported parser diagnostic; the reference file
is a build pass for ts2wasm. The remaining observable blocker is semantic
diagnostic parity for invalid `const` class members.

Problem: `constInClassExpression.ts` needs a focused implementation issue for
reporting the invalid `const` class member instead of keeping the broad
generated bucket open.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constInClassExpression.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constInClassExpression.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed. The implementation-ready follow-up is
`issues/open/5354-report-invalid-const-class-members.md`.

## Scope

In scope:

- [x] Inspect fresh smart triage for the affected file.
- [x] Confirm existing class-declaration invalid-const buckets do not cover the
      current class-expression build-pass gap as an open owner.
- [x] Split the current observable behavior into an implementation-ready child
      issue.
- [x] Preserve exact reproduction commands and representative diagnostic/AST
      evidence in the child issue.

Out of scope:

- Direct implementation from this generated bucket.
- Full class-field support.
- Full TypeScript diagnostic-code parity.

## Affected paths

Expected:

- `crates/frontend/src/parser/statements_class.rs`
- `crates/frontend/src/parser/tests.rs`
- `crates/compiler/src/lib.rs` only if diagnostic mapping belongs after parse
- `scripts/run/reference-triage.py` only if classification needs a new mapping

Do not touch:

- unrelated runtime/backend code unless focused implementation proves it is
  necessary

## Acceptance criteria

- [x] Duplicate candidates are confirmed as no-match or this issue is
      superseded/split.
- [x] The child issue contains the exact fresh `reference-triage` command.
- [x] The child issue includes failing path, current build-pass result, source
      context, visible symbols, compiler AST/resolved evidence, and TypeScript
      diagnostic evidence.
- [x] The child issue acceptance names the exact reference path and expected
      diagnostic change.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
```

Impacted commands:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constInClassExpression.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constInClassExpression.ts
```

Not run:

- `cargo fmt --all --check`; issue cleanup only, no Rust code changed
- `cargo nextest run`; issue cleanup only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5354-report-invalid-const-class-members.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/constInClassExpression.ts`

## Duplicate detection

Related done buckets exist for the class-declaration invalid-const property
case, including
`issues/done/547-implement-ClassDeclarationWithInvalidConstOnPropertyDeclaration.md`.
Those are not open implementation owners for the current class-expression
build-pass semantic diagnostic gap.

## Smart triage

### Smart triage: Build pass: constInClassExpression

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/constInClassExpression.ts`

Reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constInClassExpression.ts
```

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constInClassExpression.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
unsupported_diagcodes=
unsupported_features=
```

Source context:

```ts
// @target: es2015
let C = class {
    const a = 4;
};
```

Compiler evidence:

- Tokens include `Let`, `Ident("C")`, `Class`, `{`, `Const`, `Ident("a")`,
  `=`, `Number(4)`, `;`, `}`, `;`.
- AST construction succeeds as `ClassDecl { name: "C", body: [], ... }`.
- Resolved construction succeeds with an empty class member list.
- Visible symbol extraction reports `C` and a binding named `a`.

TypeScript oracle evidence:

```text
TS1248 at line 3, character 11: A class member cannot have the 'const' keyword.
```

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/constInClassExpression.ts
result: pass; current first blocker split to issue 5354
date: 2026-05-07

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/constInClassExpression.ts --detail --no-dashboard-data
result: pass; executed=1 build_pass=1 unsupported=0
date: 2026-05-07
```

Remaining risks:

- After issue 5354 lands, related static class declaration invalid-const cases
  should be rechecked for consistent diagnostic behavior.
