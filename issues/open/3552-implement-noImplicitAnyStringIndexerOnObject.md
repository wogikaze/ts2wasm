---
id: 3552
title: "Implement Noimplicitanystringindexeronobject"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5001]
blocks: []
created: 2026-05-01
updated: 2026-05-08
---

## Summary

Closed after refreshed evidence split the current parser boundary to
`issues/open/5478-parse-element-access-plus-equal-assignments.md`.

## Problem

Fresh triage shows this fixture parses object literals, getter/setter-shaped
properties, ordinary element access, and simple element assignment. The first
current blocker is an element-access arithmetic compound assignment:

```text
UnsupportedSyntax: expected Semicolon, got Some(PlusEqual) at 423..425
```

Problem: `noImplicitAnyStringIndexerOnObject.ts` has one current parser
boundary, split to issue 5478.

## Current failure

Representative reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts
```

Coverage window:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts --detail --no-dashboard-data
```

Observed 2026-05-08:

```text
coverage: executed=1 build_pass=0 unsupported=1 unsupported_diagcodes=UnsupportedSyntax:1 unsupported_features=object-literal:1
triage: UnsupportedSyntax expected Semicolon, got Some(PlusEqual) at 423..425
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
- [x] Child issue 5478 contains an exact `reference-triage` command
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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts
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

- [x] split to `issues/open/5478-parse-element-access-plus-equal-assignments.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts`

## Duplicate detection

- `issues/open/5311-parse-property-access-arithmetic-compound-assignments.md`
  covers property access such as `M.x += 2`, not element access
  `e['hello'] += 1`.
- `issues/open/4287-implement-stringIndexerAssignments-parser-syntax.md`
  covers class string index-signature syntax, not element-access compound
  assignment expressions.
- `issues/open/5164-parse-exponentiation-compound-assignment.md` and
  `issues/open/5178-parse-bitwise-compound-assignment-operators.md` cover
  different operators.
- Split to issue 5478.

## Smart triage

Generated 2026-05-08.

```text
### Smart triage: Triage object literal: noImplicitAnyStringIndexerOnObject

- Issue class: triage-needed
- Feature label: object-literal
- Diagnostic: UnsupportedSyntax / parser-or-frontend-unsupported
- Path: reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts
```

Current compiler message:

```text
expected Semicolon, got Some(PlusEqual) at 423..425
```

Source context:

```text
23 |   e['hello'];
24 |   e['hello'] = 'modified';
25 |   e['hello'] += 1;
26 |   e['hello'] ++;
```

Visible symbols before failure:

```text
bindings: a, b, c, foo, d, bar, e
```

Compiler evidence:

```text
tokens: ok through object literals and element-access assignment/update tokens
ast/resolved: fail at PlusEqual after element-access assignment target
```

TypeScript oracle:

```text
later diagnostic: TS2538 Type 'Dog' cannot be used as an index type at map[rover]
```

## Completion evidence

Commits:

- `...`

Validation result:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts --detail --no-dashboard-data
result: pass; reproduced current UnsupportedSyntax boundary for element-access +=
date: 2026-05-08

command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/noImplicitAnyStringIndexerOnObject.ts
result: pass; generated smart triage evidence and child issue split material
date: 2026-05-08
```

Remaining risks:

- After issue 5478 advances, the same fixture may expose postfix element-access
  update parsing, enum/key type diagnostics, unique-symbol indexing, or final
  `Dog` index-type semantic diagnostics.
