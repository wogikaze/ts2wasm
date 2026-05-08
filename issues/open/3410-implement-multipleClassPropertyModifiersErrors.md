---
id: 3410
title: "Split multipleClassPropertyModifiersErrors to duplicate static modifier issue"
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

Closed the generated `multipleClassPropertyModifiersErrors` bucket by splitting the current blocker into focused child issue #5433.

## Problem

`reference/typescript/tests/cases/compiler/multipleClassPropertyModifiersErrors.ts` currently stops at duplicate `static static p3;` class member parsing.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleClassPropertyModifiersErrors.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleClassPropertyModifiersErrors.ts
result: UnsupportedSyntax expected LeftParen, got Some(Ident("p3")) at static static p3;
date: 2026-05-08
```

## Evidence

Source:

```ts
class C {
    public public p1;
    private private p2;
    static static p3;
    public private p4;
    private public p5;
    public static p6;
    private static p7;
}
```

Compiler evidence:

```text
tokens: ok through repeated public/private/static modifiers
ast/resolved: fails at p3 with expected LeftParen, got Some(Ident("p3"))
typescript oracle: reports TS1434 Unexpected keyword or identifier at the second static
```

The current first actionable blocker is duplicate `static` class member modifier handling. Later duplicate/conflicting accessibility modifier diagnostics may need their own issue after #5433 advances.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleClassPropertyModifiersErrors.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleClassPropertyModifiersErrors.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- #5433 remains open for implementation.
