---
id: 4252
title: "Split staticModifierAlreadySeen to duplicate static modifier issue"
type: maintenance
area: frontend/parser
class: superseded
priority: P1
depends_on: [5433]
blocks: []
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Closed the generated `staticModifierAlreadySeen` bucket by splitting the current blocker into focused child issue #5433.

## Problem

`reference/typescript/tests/cases/compiler/staticModifierAlreadySeen.ts` currently stops at duplicate `static static foo = 1;` class member parsing.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/staticModifierAlreadySeen.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/staticModifierAlreadySeen.ts
result: UnsupportedSyntax expected LeftParen, got Some(Ident("foo")) at static static foo = 1;
date: 2026-05-08
```

## Evidence

Source:

```ts
class C {
    static static foo = 1;
    public static static bar() { }
}
```

Compiler evidence:

```text
tokens: ok through duplicate static modifiers on field and method members
ast/resolved: fails at foo with expected LeftParen, got Some(Ident("foo"))
typescript oracle: reports TS1434 at the second static for both members, plus TS2300 for the repeated static method case
```

## Child Issues

- #5433: report duplicate static class member modifiers.

## Validation

Issue sync and health checks:

```text
python scripts/manager.py update-issue-index
git diff --check
python scripts/manager.py update-issue-index --check
python scripts/manager.py check-issue-health
python scripts/manager.py check-issue-readiness -- --fail-ready-below 80
```

Focused reference checks:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/staticModifierAlreadySeen.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/staticModifierAlreadySeen.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- #5433 remains open for implementation.
