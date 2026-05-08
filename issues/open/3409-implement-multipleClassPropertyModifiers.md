---
id: 3409
title: "Close multipleClassPropertyModifiers to modified static field parser owner"
type: maintenance
area: frontend
class: superseded
priority: P1
depends_on: [5271]
blocks: []
created: 2026-05-01
updated: 2026-05-08
status: done
---

## Summary

Closed the generated `multipleClassPropertyModifiers` bucket as covered by implementation-ready issue #5271.

## Problem

`reference/typescript/tests/cases/compiler/multipleClassPropertyModifiers.ts` stops at the same modified static class field parser boundary already owned by #5271.

Focused coverage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleClassPropertyModifiers.ts --detail --no-dashboard-data
result: unsupported=1, unsupported_diagcodes=UnsupportedSyntax:1, unsupported_features=unknown-unsupported:1
date: 2026-05-08
```

Focused triage:

```text
command: env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleClassPropertyModifiers.ts
result: UnsupportedSyntax expected LeftParen, got Some(Ident("p1")) at public static p1;
date: 2026-05-08
```

## Evidence

Source:

```ts
class C {
    public static p1;
    static public p2;
    private static p3;
    static private p4;
}
```

Compiler evidence:

```text
tokens: ok through public/static/private class field modifiers
ast/resolved: fails at public static p1; with expected LeftParen, got Some(Ident("p1"))
typescript oracle: parses the class and reports TS1029 for invalid modifier order on static public/static private
```

The current first blocker is accepting modified static field declarations. The later invalid modifier order diagnostics are not actionable until the field parser advances.

## Owner

- #5271: parse modified static class fields.

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
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/multipleClassPropertyModifiers.ts --detail --no-dashboard-data
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/multipleClassPropertyModifiers.ts
```

Rust gates were not run because this slice only changes issue metadata.

## Completion evidence

Commits:

- filled by commit

Remaining risks:

- #5271 remains open for implementation.
