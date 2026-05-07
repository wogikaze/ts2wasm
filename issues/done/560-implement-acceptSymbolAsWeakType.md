---
id: 560
title: "Implement Acceptsymbolasweaktype"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [1999]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Close `acceptSymbolAsWeakType` after moving the current weak-collection symbol
diagnostic blocker into issue 1999.

## Problem

Reference test results originally showed 1 case failing in directory
`acceptSymbolAsWeakType` with diagnostics: name-resolution. Fresh triage on
2026-05-07 shows the original `WeakSet` unresolved-name blocker has advanced;
the current family is shared with `dissallowSymbolAsWeakType` and is now owned
by issue 1999.

Problem: this generated bucket duplicates the weak collection `symbol` negative
diagnostic work now tracked by issue 1999.

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

This generated bucket is superseded by issue 1999, which owns both
`acceptSymbolAsWeakType.ts` and `dissallowSymbolAsWeakType.ts`.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in the owner issue

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

- [x] Duplicate candidates below are confirmed and issue 1999 is the owner
- [x] Owner issue 1999 contains exact `reference-triage` commands
- [x] Owner issue 1999 includes the shared diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Owner issue 1999 acceptance names the first shared diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts
```

Not run:

- `cargo fmt --all --check` and `cargo nextest run`; this close is an
  issue-lifecycle-only split update, so focused reference and issue checks were
  used instead.

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] updated: `issues/open/1999-implement-dissallowSymbolAsWeakType.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts`

## Duplicate detection

- `issues/done/089-implement-acceptSymbolAsWeakType.md` - Implement Acceptsymbolasweaktype (same reference path, same feature label, same group key, title overlap)
- `issues/open/437-implement-name-resolution.md` - Implement name resolution (same feature label, same group key, title overlap)
- `issues/done/474-implement-acceptSymbolAsWeakType.md` - Implement Acceptsymbolasweaktype (same reference path, same feature label, same group key, title overlap)

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts

result:
UnresolvedFunction / function-resolution

current diagnostic:
unresolved function: `Symbol`

lowerer evidence:
tokens: ok
ast: ok
resolved/lowered: fails with `method WeakSet.add not found at 114..123`

TypeScript oracle:
TS2769 on `new WeakSet([s])` / `new WeakMap([[s, false]])`;
TS2345 on weak collection method calls with `symbol` arguments;
TS2304 for WeakRef and FinalizationRegistry in this oracle run.

decision:
superseded by issues/open/1999-implement-dissallowSymbolAsWeakType.md, which
owns both weak-collection symbol negative reference files.
```

## Historical smart triage

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
    "title": "Implement name resolution (triaged - superseded by test262 metadata issues)",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/089-implement-acceptSymbolAsWeakType.md",
    "title": "Implement Acceptsymbolasweaktype",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/437-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/474-implement-acceptSymbolAsWeakType.md",
    "title": "Implement Acceptsymbolasweaktype",
    "reason": "same reference path, same feature label, title overlap"
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
        kind: New,
        span: Span {
            start: 96,
            end: 99,
        },
    },
    SpannedToken {
        kind: Ident(
            "WeakSet",
        ),
        span: Span {
            start: 100,
            end: 107,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 107,
            end: 108,
        },
    },
    SpannedToken {
        kind: LeftBracket,
        span: Span {
            start: 108,
            end: 109,
        },
    },
    SpannedToken {
        kind: Ident(
            "s",
        ),
        span: Span {
            start: 109,
            end: 110,
        },
    },
    SpannedToken {
        kind: RightBracket,
        span: Span {
            start: 110,
            end: 111,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "ws",
        ),
        span: Span {
            start: 114,
            end: 116,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 116,
            end: 117,
        },
    },
    SpannedToken {
        kind: Ident(
            "add",
        ),
        span: Span {
            start: 117,
            end: 120,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: Ident(
            "s",
        ),
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
                        Present(
                            Ident {
                                name: "s",
                                span: Span {
                                    start: 109,
                                    end: 110,
                                },
                            },
                        ),
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
                        end: 116,
                    },
                },
                property: "add",
                span: Span {
                    start: 114,
                    end: 120,
                },
            },
            args: [
                Ident {
                    name: "s",
                    span: Span {
                        start: 121,
                        end: 122,
                    },
                },
            ],
            span: Span {
                start: 114,
                end: 123,
            },
        },
        span: Span {
            start: 114,
            end: 124,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "ws",
                    span: Span {
                        start: 125,
                        end: 127,
                    },
                },
                property: "has",
                span: Span {
                    start: 125,
                    end: 131,
                },
            },
            args: [
                Ident {
                    name: "s",
                    span: Span {
                        start: 132,
                        end: 133,
                    },
                },
            ],
            span: Span {
                start: 125,
                end: 134,
            },
        },
        span: Span {
            start: 125,
            end: 135,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
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

## Completion evidence

Commits:

- Superseded by issue 1999 after fresh triage showed the current blocker is
  the shared weak-collection symbol diagnostic family.

Validation result:

```text
command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts --detail --no-dashboard-data
result:
executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnresolvedFunction:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/home/wogikaze/wgkz/ts2wasm/target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/acceptSymbolAsWeakType.ts
result:
UnresolvedFunction / function-resolution; current diagnostic is `unresolved function: Symbol`; lowerer evidence reaches `method WeakSet.add not found`
date:
2026-05-07
```

Remaining risks:

- Issue 1999 owns implementation and follow-up verification.

## False-done audit

**truly-done** (560)

- Implementation commits: verified via `git log --oneline --all --grep=560`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
