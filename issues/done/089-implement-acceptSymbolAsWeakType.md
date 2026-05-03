---
id: 089
title: "Implement Acceptsymbolasweaktype (dup)"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5005]
blocks: []
created: 2026-04-29
updated: 2026-05-04
---

## Summary

Triage acceptSymbolAsWeakType across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `acceptSymbolAsWeakType` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: acceptSymbolAsWeakType has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

## Scope

In scope:

- [ ] Inspect the smart triage report below
- [ ] Confirm whether existing open/done issues already cover this bucket
- [ ] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [ ] Preserve exact reproduction commands and representative AST/diagnostic evidence in each child issue

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

- [ ] Duplicate candidates below are confirmed as no-match or this issue is superseded
- [ ] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [ ] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [ ] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected

Current state:

- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts`

## Duplicate detection

## Smart triage

### Smart triage: Triage name resolution: acceptSymbolAsWeakType

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 366,
  "lines": 23,
  "extension": ".ts",
  "first_code_line": "const s: symbol = Symbol('s');"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `WeakSet` at 100..107",
  "span_start": 100,
  "span_end": 107,
  "line": 7,
  "column": 16,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
 4 |
 5 | const s: symbol = Symbol('s');
 6 |
 7 | const ws = new WeakSet([s]);
 8 | ws.add(s);
 9 | ws.has(s);
10 | ws.delete(s);
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "s",
    "line": 5,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "ws",
    "line": 7,
    "column": 1,
    "initializer": "new"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/064-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/089-implement-acceptSymbolAsWeakType.md",
    "title": "Implement Acceptsymbolasweaktype",
    "reason": "same reference path, title overlap"
  }
]
```

Error-specific suggestions:

- Check whether the missing name should be a local binding, function binding, builtin, import binding, or runtime global.
- Acceptance should assert both the formerly missing symbol and an adjacent negative case.

Automatic repair sketch:

```rust
// Rough sketch only: make unresolved names inspectable at resolver failure.
if let Some(binding) = self.lookup_name(name) {
    return Ok(binding);
}
return Err(Diagnostic {
    code: DiagCode::UnresolvedName,
    message: format!("unresolved name `{name}`; visible bindings: {:?}", self.visible_names()),
    span,
});
```

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Const,
        span: Span {
            start: 53,
            end: 58,
        },
    },
    SpannedToken {
        kind: Ident(
            "s",
        ),
        span: Span {
            start: 59,
            end: 60,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 60,
            end: 61,
        },
    },
    SpannedToken {
        kind: Ident(
            "symbol",
        ),
        span: Span {
            start: 62,
            end: 68,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 69,
            end: 70,
        },
    },
    SpannedToken {
        kind: Ident(
            "Symbol",
        ),
        span: Span {
            start: 71,
            end: 77,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: String(
            "s",
        ),
        span: Span {
            start: 78,
            end: 81,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 81,
            end: 82,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 85,
            end: 90,
        },
    },
    SpannedToken {
        kind: Ident(
            "ws",
        ),
        span: Span {
            start: 91,
            end: 93,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 94,
            end: 95,
        },
    },
    SpannedToken {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "s",
        expr: Call {
            callee: Ident {
                name: "Symbol",
                span: Span {
                    start: 71,
                    end: 77,
                },
            },
            args: [
                String {
                    value: "s",
                    span: Span {
                        start: 78,
                        end: 81,
                    },
                },
            ],
            span: Span {
                start: 71,
                end: 82,
            },
        },
        span: Span {
            start: 53,
            end: 83,
        },
    },
    Let {
        name: "ws",
        expr: New {
            expr: Ident {
                name: "WeakSet",
                span: Span {
                    start: 100,
                    end: 107,
                },
            },
            args: [
                Array {
                    elements: [
                        Ident {
                            name: "s",
                            span: Span {
                                start: 109,
                                end: 110,
                            },
                        },
                    ],
                    span: Span {
                        start: 108,
                        end: 111,
                    },
                },
            ],
            span: Span {
                start: 96,
                end: 112,
            },
        },
        span: Span {
            start: 85,
            end: 113,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "ws",
                    span: Span {
                        start: 114,
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `WeakSet` at 100..107
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": false,
    "diagnostics": [
      {
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 2, '(iterable: Iterable<object>): WeakSet<object>', gave the following error.\n    Argument of type 'symbol[]' is not assignable to parameter of type 'Iterable<object>'.\n      The types returned by '[Symbol.iterator]().next(...)' are incompatible between these types.\n        Type 'IteratorResult<symbol, undefined>' is not assignable to type 'IteratorResult<object, any>'.\n          Type 'IteratorYieldResult<symbol>' is not assignable to type 'IteratorResult<object, any>'.\n            Type 'IteratorYieldResult<symbol>' is not assignable to type 'IteratorYieldResult<object>'.\n              Type 'symbol' is not assignable to type 'object'.\n  Overload 2 of 2, '(values?: readonly object[] | null | undefined): WeakSet<object>', gave the following error.\n    Type 'symbol' is not assignable to type 'object'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 100,
        "length": 7,
        "line": 7,
        "character": 16
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type 'symbol' is not assignable to parameter of type 'object'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 121,
        "length": 1,
        "line": 8,
        "character": 8
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type 'symbol' is not assignable to parameter of type 'object'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 132,
        "length": 1,
        "line": 9,
        "character": 8
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type 'symbol' is not assignable to parameter of type 'object'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 146,
        "length": 1,
        "line": 10,
        "character": 11
      },
      {
        "code": 2769,
        "category": "Error",
        "message": "No overload matches this call.\n  Overload 1 of 2, '(iterable?: Iterable<readonly [object, boolean]> | null | undefined): WeakMap<object, boolean>', gave the following error.\n    Argument of type '[symbol, false][]' is not assignable to parameter of type 'Iterable<readonly [object, boolean]>'.\n      The types returned by '[Symbol.iterator]().next(...)' are incompatible between these types.\n        Type 'IteratorResult<[symbol, false], undefined>' is not assignable to type 'IteratorResult<readonly [object, boolean], any>'.\n          Type 'IteratorYieldResult<[symbol, false]>' is not assignable to type 'IteratorResult<readonly [object, boolean], any>'.\n            Type 'IteratorYieldResult<[symbol, false]>' is not assignable to type 'IteratorYieldResult<readonly [object, boolean]>'.\n              Type '[symbol, false]' is not assignable to type 'readonly [object, boolean]'.\n                Type at position 0 in source is not compatible with type at position 0 in target.\n                  Type 'symbol' is not assignable to type 'object'.\n  Overload 2 of 2, '(entries?: readonly (readonly [object, boolean])[] | null | undefined): WeakMap<object, boolean>', gave the following error.\n    Type 'symbol' is not assignable to type 'object'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 166,
        "length": 7,
        "line": 12,
        "character": 16
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type 'symbol' is not assignable to parameter of type 'object'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 196,
        "length": 1,
        "line": 13,
        "character": 8
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type 'symbol' is not assignable to parameter of type 'object'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 213,
        "length": 1,
        "line": 14,
        "character": 8
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type 'symbol' is not assignable to parameter of type 'object'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 224,
        "length": 1,
        "line": 15,
        "character": 8
      },
      {
        "code": 2345,
        "category": "Error",
        "message": "Argument of type 'symbol' is not assignable to parameter of type 'object'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 238,
        "length": 1,
        "line": 16,
        "character": 11
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'WeakRef'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 258,
        "length": 7,
        "line": 18,
        "character": 16
      },
      {
        "code": 2304,
        "category": "Error",
        "message": "Cannot find name 'FinalizationRegistry'.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 297,
        "length": 20,
        "line": 21,
        "character": 15
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "symbol",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 59,
        "length": 1,
        "line": 5,
        "character": 7,
        "name": "s"
      },
      {
        "kind": "binding",
        "typeText": "WeakSet<object>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 91,
        "length": 2,
        "line": 7,
        "character": 7,
        "name": "ws"
      },
      {
        "kind": "binding",
        "typeText": "WeakMap<object, boolean>",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 157,
        "length": 2,
        "line": 12,
        "character": 7,
        "name": "wm"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 249,
        "length": 2,
        "line": 18,
        "character": 7,
        "name": "wr"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts",
        "start": 289,
        "length": 1,
        "line": 21,
        "character": 7,
        "name": "f"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "const s: symbol = Symbol('s');",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const ws = new WeakSet([s]);",
        "line": 7,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "ws.add(s);",
        "line": 8,
        "c
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `WeakSet` at 100..107
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/560-implement-acceptSymbolAsWeakType.md` に統合されました。
そちらを参照してください。
## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
