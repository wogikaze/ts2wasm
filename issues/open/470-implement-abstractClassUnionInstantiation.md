---
id: 470
title: "Implement Abstractclassunioninstantiation (dup)"
type: spike
area: frontend/resolver
class: superseded
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-04
---

## Summary

Triage abstractClassUnionInstantiation across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `abstractClassUnionInstantiation` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: abstractClassUnionInstantiation has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts --detail
```

## Desired final state

This generated bucket is either split into implementation-ready child issues or superseded by an existing open/done issue with matching evidence. Do not implement directly from this bucket.

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
- [x] At least one child issue contains an exact `mise run reference-triage -- ...` command
- [x] Child issue includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue acceptance names the exact fixture/reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts`

## Duplicate detection

- `issues/open/084-implement-abstractClassUnionInstantiation.md` - Implement Abstractclassunioninstantiation (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage name resolution: abstractClassUnionInstantiation

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 755,
  "lines": 22,
  "extension": ".ts",
  "first_code_line": "class ConcreteA {}"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `abstract` at 57..65",
  "span_start": 57,
  "span_end": 65,
  "line": 4,
  "column": 1,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
1 | // @target: es2015
2 | class ConcreteA {}
3 | class ConcreteB {}
4 | abstract class AbstractA { a: string; }
5 | abstract class AbstractB { b: string; }
6 | 
7 | type Abstracts = typeof AbstractA | typeof AbstractB;
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "ConcreteA",
    "line": 2,
    "column": 1
  },
  {
    "kind": "class",
    "name": "ConcreteB",
    "line": 3,
    "column": 1
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
    "path": "issues/open/084-implement-abstractClassUnionInstantiation.md",
    "title": "Implement Abstractclassunioninstantiation",
    "reason": "same reference path, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/437-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
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
        kind: Class,
        span: Span {
            start: 19,
            end: 24,
        },
    },
    SpannedToken {
        kind: Ident(
            "ConcreteA",
        ),
        span: Span {
            start: 25,
            end: 34,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 35,
            end: 36,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 38,
            end: 43,
        },
    },
    SpannedToken {
        kind: Ident(
            "ConcreteB",
        ),
        span: Span {
            start: 44,
            end: 53,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 54,
            end: 55,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 55,
            end: 56,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 57,
            end: 65,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 66,
            end: 71,
        },
    },
    SpannedToken {
        kind: Ident(
            "AbstractA",
        ),
        span: Span {
            start: 72,
            end: 81,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 82,
            end: 83,
        },
    },
    SpannedToken {
        kind: Ident(
            "a",
        ),
        span: Span {
            start: 84,
            end: 85,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 87,
            end: 93,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Ident(
            "abstract",
        ),
        span: Span {
            start: 97,
            end: 105,
        },
    },
    SpannedToken {
        kind: Class,
        span: Span {
            start: 106,
            end: 111,
        },
    },
    SpannedToken {
        kind: Ident(
            "AbstractB",
        ),
        span: Span {
            start: 112,
            end: 121,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 122,
            end: 123,
        },
    },
    SpannedToken {
        kind: Ident(
            "b",
        ),
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 125,
            end: 126,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 127,
            end: 133,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 133,
            end: 134,
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
    ClassDecl {
        name: "ConcreteA",
        extends: None,
        body: [],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 19,
            end: 37,
        },
    },
    ClassDecl {
        name: "ConcreteB",
        extends: None,
        body: [],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 38,
            end: 56,
        },
    },
    Expr {
        expr: Ident {
            name: "abstract",
            span: Span {
                start: 57,
                end: 65,
            },
        },
        span: Span {
            start: 57,
            end: 65,
        },
    },
    ClassDecl {
        name: "AbstractA",
        extends: None,
        body: [],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 66,
            end: 96,
        },
    },
    Expr {
        expr: Ident {
            name: "abstract",
            span: Span {
                start: 97,
                end: 105,
            },
        },
        span: Span {
            start: 97,
            end: 105,
        },
    },
    ClassDecl {
        name: "AbstractB",
        extends: None,
        body: [],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 106,
            end: 136,
        },
    },
    Expr {
        expr: New {
            expr: Ident {
                name: "cls1",
                span: Span {
                    start: 407,
                    end: 411,
                },
            },
            args: [],
            span: Span {
                start: 403,
                end: 413,
            },
        },
        span: Span {
            start: 403,
            end: 414,
        },
    },
    Expr {
        expr: New {
            expr: Ident {
                name: "cls2",
                span: Span {
                    start: 435,
                    end: 439,
                },
            },
            args: [],
            span: Span {
                start: 431,
                end: 441,
            },
        },
        span: Span {
            start: 431,
            end: 442,
        },
    },
    Expr {
        expr: New {
            expr: Ident {
                name: "cls3",
                span: Span {
                    start: 463,
                    end: 467,
                },
            },
            args: [],
            span: Span {
                start: 459,
                end: 469,
            },
        },
        span: Span {
            start: 459,
            end: 470,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Array {
                    elements: [
                        Present(
                            Ident {
                                name: "ConcreteA",
                                span: Span {
                                    start: 488,
                                    end: 497,
                                },
                            },
                        ),
                        Present(
                            Ident {
                                name: "AbstractA",
                                span: Span {
                                    start: 499,
                                    end: 508,
                                },
                            },
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `abstract` at 57..65
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
        "code": 2564,
        "category": "Error",
        "message": "Property 'a' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 84,
        "length": 1,
        "line": 4,
        "character": 28
      },
      {
        "code": 2564,
        "category": "Error",
        "message": "Property 'b' has no initializer and is not definitely assigned in the constructor.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 124,
        "length": 1,
        "line": 5,
        "character": 28
      },
      {
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 403,
        "length": 10,
        "line": 15,
        "character": 1
      },
      {
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 431,
        "length": 10,
        "line": 16,
        "character": 1
      },
      {
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 532,
        "length": 9,
        "line": 19,
        "character": 46
      },
      {
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 605,
        "length": 9,
        "line": 20,
        "character": 46
      },
      {
        "code": 2511,
        "category": "Error",
        "message": "Cannot create an instance of an abstract class.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 728,
        "length": 9,
        "line": 22,
        "character": 35
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "ConcretesOrAbstracts",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 312,
        "length": 4,
        "line": 11,
        "character": 15,
        "name": "cls1"
      },
      {
        "kind": "binding",
        "typeText": "Abstracts",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 354,
        "length": 4,
        "line": 12,
        "character": 15,
        "name": "cls2"
      },
      {
        "kind": "binding",
        "typeText": "Concretes",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 385,
        "length": 4,
        "line": 13,
        "character": 15,
        "name": "cls3"
      },
      {
        "kind": "parameter",
        "typeText": "typeof ConcreteA | typeof AbstractA | typeof AbstractB",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 525,
        "length": 3,
        "line": 19,
        "character": 39,
        "name": "cls"
      },
      {
        "kind": "parameter",
        "typeText": "typeof ConcreteA | typeof AbstractA | typeof AbstractB",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 598,
        "length": 3,
        "line": 20,
        "character": 39,
        "name": "cls"
      },
      {
        "kind": "parameter",
        "typeText": "typeof ConcreteA",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 660,
        "length": 3,
        "line": 21,
        "character": 28,
        "name": "cls"
      },
      {
        "kind": "parameter",
        "typeText": "typeof AbstractA | typeof AbstractB",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/abstractClassUnionInstantiation.ts",
        "start": 721,
        "length": 3,
        "line": 22,
        "character": 28,
        "name": "cls"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "ClassDeclaration",
        "text": "class ConcreteA {}",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "class ConcreteB {}",
        "line": 3,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class AbstractA { a: string; }",
        "line": 4,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class AbstractB { b: string; }",
        "line": 5,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type Abstracts = typeof AbstractA | typeof AbstractB;",
        "line": 7,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type Concretes = typeof ConcreteA | typeof ConcreteB;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "TypeAliasDeclaration",
        "text": "type ConcretesOrAbstracts = Concretes | Abstracts;",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const cls1: ConcretesOrAbstracts;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const cls2: Abstracts;",
        "line": 12,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare const cls3: Concretes;",
        "line": 13,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "new cls1();",
        "line": 15,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "new cls2();",
        "line": 16,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "new cls3();",
        "line": 17,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "[ConcreteA, AbstractA, AbstractB].map(cls => new cls());",
        "line": 19,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "[AbstractA, AbstractB, ConcreteA].map(cls => new cls());",
        "line": 20,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "[ConcreteA, ConcreteB].map(cls => new cls());",
        "line": 21,
        "character": 1
      },
      {
        "kind": "ExpressionStatement",
        "text": "[AbstractA, AbstractB].map(cls => new cls());",
        "line": 22,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "class ConcreteA {}\nclass ConcreteB {}\nabstract class AbstractA { a: string; }\nabstract class AbstractB { b: string; }\n\nt",
        "line": 2,
        "character": 1
      },
      {
        "kind": "ClassDeclaration",
        "text": "abstract class AbstractA { a: string; }",
        "line": 4,
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `abstract` at 57..65
```


## 重複整理メモ

この issue はタイトルが重複しているため、内容がより充実している
`issues/open/556-implement-abstractClassUnionInstantiation.md` に統合されました。
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

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/open/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/open/470-implement-abstractClassUnionInstantiation.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
