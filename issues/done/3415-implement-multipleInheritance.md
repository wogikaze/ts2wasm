---
id: 3415
title: "Close multipleInheritance to multiple class heritage owner"
type: maintenance
area: frontend/parser
class: done
priority: P1
depends_on: []
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
status: done
---

## Summary

Closed the generated `multipleInheritance` bucket as superseded by
`issues/open/5317-report-multiple-class-heritage-bases.md`.

## Problem

Fresh triage shows the first blocker is the same parser boundary already owned
by issue 5317: a class `extends` clause with multiple base expressions.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleInheritance.ts --detail --no-dashboard-data
result: executed=1, unsupported=1, blocked=0, unsupported_diagcodes=UnsupportedSyntax:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleInheritance.ts
result: UnsupportedSyntax expected LeftBrace, got Some(Comma) at 121..122
date: 2026-05-08
```

## Evidence

Source context:

```ts
class B1 {
    public x;
}

class B2 {
    public x;
}

class C extends B1, B2 { // duplicate member
}

class E extends D1, D2 { // nope, duplicate member
}
```

Compiler evidence:

```text
tokens: ok through Class C, Extends, Ident("B1"), Comma, Ident("B2")
ast: fails with UnsupportedSyntax expected LeftBrace, got Some(Comma) at 121..122
resolved: fails with the same parser diagnostic
visible symbols before failure: class B1, class B2, class C
```

TypeScript oracle evidence:

```text
TS1174 at class C extends B1, B2: Classes can only extend a single class.
TS1174 at class E extends D1, D2: Classes can only extend a single class.
Additional later diagnostics: TS2564, TS2425, TS2416.
TypeScript AST includes ClassDeclaration C with HeritageClause "extends B1, B2".
```

Issue 5317 already scopes the exact TS1174 boundary and now includes this
reference path in its evidence and acceptance criteria.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleInheritance.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleInheritance.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- Implementation remains open in issue 5317.
