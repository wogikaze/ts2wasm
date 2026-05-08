---
id: 1306
title: "Implement Collisionrestparameterunderscoreiusage"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1306.

## Summary

Triage collisionRestParameterUnderscoreIUsage across 1 reference case and close
it after splitting the current rest-constructor lexical-capture blocker into an
implementation-ready child issue.

## Problem

Reference test results previously showed 1 case failing in directory
`collisionRestParameterUnderscoreIUsage` with diagnostics: class. Fresh triage
shows tokens and AST now succeed; the current blocker is issue-289 for a class
constructor with a rest parameter capturing outer local `_i`.

Problem: constructor rest parameters and hidden lexical-capture parameters
cannot currently coexist in the lowered call ABI.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed after splitting the current observable blocker
into `issues/open/5338-support-rest-constructor-outer-local-captures.md`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split the rest-constructor lexical-capture blocker into a child issue
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
- [x] Child issue 5338 contains an exact reference-triage command
- [x] Child issue 5338 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue 5338 acceptance names the exact reference path and diagnostic change

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
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts --detail --no-dashboard-data
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts
```

Not run:

- `cargo fmt --all --check`; issue split/close only, no Rust code changed
- `cargo nextest run`; issue split/close only, no implementation changed

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] created: `issues/open/5338-support-rest-constructor-outer-local-captures.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts`

## Duplicate detection

- `issues/open/5152-support-class-constructor-outer-callback-captures.md` - related constructor capture support, but does not own rest-parameter constructor ABI collision
-  - related issue-289 constructor capture, but covers later class binding
- no exact existing owner found for rest-parameter constructor hidden-capture ABI collision

## Smart triage

Fresh triage shows this generated class bucket is currently blocked by
rest-constructor outer-local capture ABI handling.

### Smart triage: Triage class: collisionRestParameterUnderscoreIUsage

- Issue class: `triage-needed`
- Feature label: `class`
- Diagnostic: `UnsupportedSyntax` / `parser-or-frontend-unsupported`
- Path: `reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts
```

Coverage:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=0
semantic_pass=0
unsupported=1
blocked=0
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=unknown-unsupported:1
semantic_enabled=0
```

Source context:

```ts
var _i = "This is what I'd expect to see";
class Foo {
    constructor(...args: any[]) {
        console.log(_i);
    }
}
new Foo();
```

Compiler evidence:

```text
tokens: ok; includes class Foo, constructor, DotDotDot rest parameter args, console.log(_i), and new Foo()
ast: ok; ClassDecl Foo has constructor rest parameter args and call console.log(_i)
resolved: UnsupportedSyntax issue-289 for captured outer local _i with a rest parameter
visible symbols: console, _i, and class Foo
```

TypeScript oracle evidence:

```text
TS2403: Subsequent variable declarations must have the same type for console
```

Split result:

- `issues/open/5338-support-rest-constructor-outer-local-captures.md`

## Completion evidence

Commits:

- Split to `issues/open/5338-support-rest-constructor-outer-local-captures.md`.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; issue-289 rest-constructor capture blocker split to issue 5338
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/collisionRestParameterUnderscoreIUsage.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; UnsupportedSyntax/unknown-unsupported
date: 2026-05-07
```

Remaining risks:

- After issue 5338 lands, this reference may expose the TypeScript TS2403
  console redeclaration diagnostic as the next semantic blocker.
