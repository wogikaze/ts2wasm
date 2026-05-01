---
id: 640
title: "Implement Anyandunknownhavefalsycomponents"
type: spike
area: frontend/resolver
class: blocked
priority: P1
depends_on: [5005]
blocks: []
created: 2026-05-01
updated: 2026-05-01
---

## Summary

Triage anyAndUnknownHaveFalsyComponents across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `anyAndUnknownHaveFalsyComponents` with diagnostics: name-resolution. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: anyAndUnknownHaveFalsyComponents has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts --detail
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
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts
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

- `reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts`

## Duplicate detection

- `issues/open/182-implement-anyAndUnknownHaveFalsyComponents.md` - Implement Anyandunknownhavefalsycomponents (same reference path, same group key, title overlap)

## Smart triage

### Smart triage: Triage name resolution: anyAndUnknownHaveFalsyComponents

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 454,
  "lines": 30,
  "extension": ".ts",
  "first_code_line": "declare let x1: any;"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `x1` at 79..81",
  "span_start": 79,
  "span_end": 81,
  "line": 5,
  "column": 12,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
2 | // @strictNullChecks: true
3 | 
4 | declare let x1: any;
5 | const y1 = x1 && 3;
6 | 
7 | // #39113
8 | declare let isTreeHeader1: any;
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "x1",
    "line": 4,
    "column": 9
  },
  {
    "kind": "binding",
    "name": "y1",
    "line": 5,
    "column": 1,
    "initializer": ""
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
    "path": "issues/open/182-implement-anyAndUnknownHaveFalsyComponents.md",
    "title": "Implement Anyandunknownhavefalsycomponents",
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
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 47,
            end: 54,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 55,
            end: 58,
        },
    },
    SpannedToken {
        kind: Ident(
            "x1",
        ),
        span: Span {
            start: 59,
            end: 61,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 61,
            end: 62,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 63,
            end: 66,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: Const,
        span: Span {
            start: 68,
            end: 73,
        },
    },
    SpannedToken {
        kind: Ident(
            "y1",
        ),
        span: Span {
            start: 74,
            end: 76,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 77,
            end: 78,
        },
    },
    SpannedToken {
        kind: Ident(
            "x1",
        ),
        span: Span {
            start: 79,
            end: 81,
        },
    },
    SpannedToken {
        kind: AndAnd,
        span: Span {
            start: 82,
            end: 84,
        },
    },
    SpannedToken {
        kind: Number(
            3,
        ),
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 86,
            end: 87,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 99,
            end: 106,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 107,
            end: 110,
        },
    },
    SpannedToken {
        kind: Ident(
            "isTreeHeader1",
        ),
        span: Span {
            start: 111,
            end: 124,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 124,
            end: 125,
        },
    },
    SpannedToken {
        kind: Ident(
            "any",
        ),
        span: Span {
            start: 126,
            end: 129,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 129,
            end: 130,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 131,
            end: 139,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo1",
        ),
        span: Span {
            start: 140,
            end: 144,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 144,
            end: 145,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 145,
            end: 146,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 147,
            end: 148,
        },
    },
    SpannedToken {
        kind: Return,
        span: Span {
            start: 151,
            end: 157,
        },
    },
    SpannedToken {
        kind: LeftBrace,
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "y1",
        expr: Binary {
            left: Ident {
                name: "x1",
                span: Span {
                    start: 79,
                    end: 81,
                },
            },
            op: And,
            right: Number {
                value: 3,
                span: Span {
                    start: 85,
                    end: 86,
                },
            },
            span: Span {
                start: 79,
                end: 86,
            },
        },
        span: Span {
            start: 68,
            end: 87,
        },
    },
    Function {
        name: "foo1",
        params: [],
        body: [
            Return {
                expr: Object {
                    props: [
                        (
                            "display",
                            String {
                                value: "block",
                                span: Span {
                                    start: 173,
                                    end: 180,
                                },
                            },
                        ),
                        (
                            "\0ts2wasm_object_spread",
                            Binary {
                                left: Ident {
                                    name: "isTreeHeader1",
                                    span: Span {
                                        start: 190,
                                        end: 203,
                                    },
                                },
                                op: And,
                                right: Object {
                                    props: [
                                        (
                                            "display",
                                            String {
                                                value: "flex",
                                                span: Span {
                                                    start: 224,
                                                    end: 230,
                                                },
                                            },
                                        ),
                                    ],
                                    span: Span {
                                        start: 207,
                                        end: 237,
                                    },
                                },
                                span: Span {
                                    start: 190,
                                    end: 237,
                                },
                            },
                        ),
                    ],
                    span: Span {
                        start: 158,
                        end: 242,
                    },
                },
                span: Span {
                    start: 151,
                    end: 243,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 131,
            end: 243,
        },
    },
    Let {
        name: "y2",
        expr: Binary {
            left: Ident {
                name: "x2",
                span: Span {
                    start: 283,
                    end: 285,
                },
            },
            op: And,
            right: Number {
                valu
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `x1` at 79..81
```

TypeScript/JavaScript oracle:

```json
{
  "ok": true,
  "returncode": 0,
  "typescript": {
    "ok": true,
    "diagnostics": [],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 59,
        "length": 2,
        "line": 4,
        "character": 13,
        "name": "x1"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 74,
        "length": 2,
        "line": 5,
        "character": 7,
        "name": "y1"
      },
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 111,
        "length": 13,
        "line": 8,
        "character": 13,
        "name": "isTreeHeader1"
      },
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 140,
        "length": 4,
        "line": 9,
        "character": 10,
        "name": "foo1"
      },
      {
        "kind": "binding",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 259,
        "length": 2,
        "line": 18,
        "character": 13,
        "name": "x2"
      },
      {
        "kind": "binding",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 278,
        "length": 2,
        "line": 19,
        "character": 7,
        "name": "y2"
      },
      {
        "kind": "binding",
        "typeText": "unknown",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 315,
        "length": 13,
        "line": 22,
        "character": 13,
        "name": "isTreeHeader2"
      },
      {
        "kind": "function",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/anyAndUnknownHaveFalsyComponents.ts",
        "start": 348,
        "length": 4,
        "line": 23,
        "character": 10,
        "name": "foo2"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "FirstStatement",
        "text": "declare let x1: any;",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const y1 = x1 && 3;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare let isTreeHeader1: any;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo1() {\n  return {\n    display: \"block\",\n    ...(isTreeHeader1 && {\n      display: \"flex\",\n    })\n  };\n}",
        "line": 9,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare let x2: unknown;",
        "line": 18,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const y2 = x2 && 3;",
        "line": 19,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "declare let isTreeHeader2: unknown;",
        "line": 22,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "function foo2() {\n  return {\n    display: \"block\",\n    ...(isTreeHeader1 && {\n      display: \"flex\",\n    })\n  };\n}",
        "line": 23,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "declare let x1: any;\nconst y1 = x1 && 3;\n\n// #39113\ndeclare let isTreeHeader1: any;\nfunction foo1() {\n  return {\n    dis",
        "line": 4,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "const y1 = x1 && 3;",
        "line": 5,
        "character": 1
      },
      {
        "kind": "VariableDeclarationList",
        "text": "const y1 = x1 && 3",
        "line": 5,
        "character": 1
      },
      {
        "kind": "VariableDeclaration",
        "text": "y1 = x1 && 3",
        "line": 5,
        "character": 7
      },
      {
        "kind": "BinaryExpression",
        "text": "x1 && 3",
        "line": 5,
        "character": 12
      },
      {
        "kind": "Identifier",
        "text": "x1",
        "line": 5,
        "character": 12
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `x1` at 79..81
```

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
