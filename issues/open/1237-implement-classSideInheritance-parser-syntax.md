---
id: 1237
title: "Implement Classsideinheritance Parser Syntax"
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
> Evidence: Empty completion evidence. No feat/fix commit for #1237.

## Summary

Closed as stale. Fresh triage and focused coverage show
`reference/typescript/tests/cases/compiler/classSideInheritance2.ts` now
build-passes, so there is no current parser-syntax blocker to split into a
child issue.

## Problem

Reference test results previously showed 1 case failing in directory
`classSideInheritance-parser-syntax` with diagnostics: parser-syntax. Fresh
triage shows tokens, AST, resolution, and build all succeed.

Problem: the generated parser-syntax bucket is stale. TypeScript still reports
TS2449 for class `TextBase` used before its declaration and TS2564 for an
uninitialized property, but those are semantic parity diagnostics, not the
original parser-syntax build blocker.

## Current failure

Representative reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classSideInheritance2.ts
```

Coverage window:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance2.ts --detail --no-dashboard-data
```

## Desired final state

This generated bucket is closed as stale because the representative path now
reports `build_pass`. Do not implement directly from this bucket.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm the representative case is now build-pass with no active parser blocker
- [x] Close as stale build-pass instead of creating a child issue
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in this closed issue

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

- [x] Duplicate candidates below are confirmed; no separate issue is needed
- [x] No child issue needed because the representative case now build-passes
- [x] This issue includes path, diagnostic status, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Completion evidence names the exact reference path and build-pass result

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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance2.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/classSideInheritance2.ts
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

- [x] none; current compiler build has no parser-syntax blocker on this path

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/classSideInheritance2.ts`

## Duplicate detection

No exact implementation child is created because the original parser-syntax
blocker no longer reproduces. Broad parser-syntax issues such as
`issues/open/442-implement-parser-syntax.md` and
`issues/open/059-implement-parser-syntax-extensions.md` are not exact owners for
this current state: this file now parses, resolves, and builds.

Resolution:

```text
The original parser-syntax blocker is stale. The reference window now reports build_pass with semantic checking disabled, so no implementation-ready blocker is split from this generated bucket.
```

## Smart triage

### Smart triage: Build pass: classSideInheritance2

- Issue class: `none`
- Feature label: `build-pass`
- Diagnostic: `BuildPass` / `pass`
- Path: `reference/typescript/tests/cases/compiler/classSideInheritance2.ts`

Reproduction:

```sh
python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classSideInheritance2.ts
```

Coverage reproduction:

```sh
python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance2.ts --detail --no-dashboard-data
```

Coverage result:

```text
executed=1
build_pass=1
unsupported=0
blocked=0
unsupported_features=
```

Source context:

```ts
interface IText {
    foo: number;
}

interface TextSpan {}

class SubText extends TextBase {
    constructor(text: IText, span: TextSpan) {
        super();
    }
}

class TextBase implements IText {
    public foo: number;
    public subText(span: TextSpan): IText {
        return new SubText(this, span);
    }
}
```

Compiler evidence:

```text
tokens: ok; includes `class SubText extends TextBase` and `class TextBase implements IText`
ast: ok; ClassDecl SubText extends TextBase, ClassDecl TextBase, method subText returns new SubText(this, span)
resolved: ok; SubText extends TextBase, TextBase.subText captures SubText and resolves new SubText(this, span)
```

TypeScript oracle evidence:

```text
TS2449: Class 'TextBase' used before its declaration.
TS2564: Property 'foo' has no initializer and is not definitely assigned in the constructor.
```

## Completion evidence

Fill only when moving to `done/`.

Commits:

- Closed as stale; see local commit for this issue cleanup.

Validation result:

```text
command: python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/classSideInheritance2.ts
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; build succeeded and original parser-syntax blocker is stale
date: 2026-05-07

command: python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/classSideInheritance2.ts --detail --no-dashboard-data
result: pass on the main checkout with TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm; executed=1, build_pass=1, unsupported=0
date: 2026-05-07
```

Remaining risks:

- TypeScript still reports TS2449 and TS2564 semantic diagnostics; those are
  future semantic parity gaps, not the generated parser-syntax blocker closed
  here.
