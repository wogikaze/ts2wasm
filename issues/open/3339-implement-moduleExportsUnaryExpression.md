---
id: 3339
title: "Implement Moduleexportsunaryexpression"
type: spike
area: reference/triage
class: split
priority: P2
depends_on: []
blocks: [5415]
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed this generated import/export bucket by splitting its current first
blocker into `issues/open/5415a-support-identifier-update-expressions-in-value-positions.md`.

## Problem

Fresh triage shows the file parses `export function`, the function body update
expressions, and the final `export { x }` syntax, but the resolver/lowering
path rejects identifier update expressions when their value is used in a binary
expression or return expression.

## Current failure

Coverage reproduction:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleExportsUnaryExpression --detail --no-dashboard-data
```

Observed result:

```text
executed=1
build_pass=0
unsupported=1
unsupported_diagcodes=UnsupportedSyntax:1
unsupported_features=import-export:1
```

Focused triage:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExportsUnaryExpression.ts
```

Source shape:

```ts
let x = 1;

export function foo(y: number) {
    if (y <= x++) return y <= x++;
    if (y <= x--) return y <= x--;
    if (y <= ++x) return y <= ++x;
    if (y <= --x) return y <= --x;

    x++;
    x--;
    ++x;
    --x;
}

export { x };
```

Compiler evidence:

```text
tokens: ok through update expressions and export declarations
ast: ok; `x++`, `x--`, `++x`, and `--x` are represented as Unary expressions
smart triage: UnsupportedSyntax issue-268 at `y <= x++`
resolved dump: later `export { x }` named-export boundary is also visible
```

TypeScript oracle evidence:

```text
ok: true
diagnostics: []
```

## Desired final state

Implement issue 5415 or re-triage after it lands. Do not implement directly
from this generated bucket.

## Scope

In scope:

- [x] Confirm current first compiler blocker.
- [x] Split value-position identifier update expression support into a focused issue.
- [x] Preserve exact reproduction commands and later named-export risk.

Out of scope:

- Direct implementation from this generated bucket.
- Reworking completed for-loop update-slot support from issue 268.
- Broad module graph behavior.

## Affected paths

Expected implementation owner:

- `crates/ir/src/`
- `crates/frontend/src/`
- focused CLI/IR tests

## Acceptance criteria

- [x] Current blocker is represented in `issues/open/5415a-support-identifier-update-expressions-in-value-positions.md`.
- [x] Existing issue 5181 is documented as related but narrower than this postfix value-position failure.
- [x] The generated bucket no longer remains as a stale blocked import/export issue.

## Validation

Required commands:

```sh
python scripts/manager.py update-issue-index
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
git diff --check
git diff --cached --check
```

Reference commands already run:

```sh
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/moduleExportsUnaryExpression --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/moduleExportsUnaryExpression.ts
```

Not run:

- `cargo fmt --all --check` (issue lifecycle only; no Rust changes)
- `cargo nextest run` (issue lifecycle only; no Rust changes)

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] `issues/open/5415a-support-identifier-update-expressions-in-value-positions.md`

## Notes

Split to issue 5415. After update-expression support lands, the final
`export { x }` should be rechecked; local named export list support is covered
by completed issues 5009/5010, but the current dump still records a later
unsupported named-export boundary.

## Completion evidence

Commits:

- filled by commit

Validation result:

```text
command: python scripts/manager.py update-issue-index
result: pass
date: 2026-05-08

command: python scripts/manager.py update-issue-index --check
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-health
result: pass
date: 2026-05-08

command: python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
result: pass
date: 2026-05-08

command: git diff --check
result: pass
date: 2026-05-08

command: git diff --cached --check
result: pass
date: 2026-05-08
```

Remaining risks:

- After issue 5415 lands, this path may expose a named-export rewrite boundary
  or broader CommonJS module behavior.
