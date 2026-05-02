---
id: 5020
title: "Implement RegExp literal support"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

Triage regexp-literal feature across 39 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 39 cases fail with regexp-literal diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: regexp-literal feature has 39 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js --detail
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
mise run reference-coverage -- test262 --limit 78
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js
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

- `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-not-capturing.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-invalid-control-escape-character-class.js`
- `reference/test262/test/annexB/built-ins/RegExp/RegExp-invalid-control-escape-character-class-range.js`
- `reference/test262/test/annexB/built-ins/RegExp/incomplete_hex_unicode_escape.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/index/prop-desc.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/lastMatch/prop-desc.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/index/this-subclass-constructor.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/input/this-not-regexp-constructor.js`
- `reference/test262/test/annexB/built-ins/RegExp/legacy-accessors/lastMatch/this-subclass-constructor.js`
- ... and 29 more files

## Duplicate detection

- `issues/open/066-implement-regexp-literal.md` - Implement RegExp literal support (same reference path, same feature label, same group key, title overlap)
- `issues/open/1139-implement-checkJsxNotSetError.md` - Implement Checkjsxnotseterror (same feature label, same group key, title overlap)
- `issues/open/2230-implement-excessiveStackDepthFlatArray.md` - Implement Excessivestackdepthflatarray (same feature label, same group key, title overlap)
- `issues/open/2872-implement-initializedDestructuringAssignmentTypes.md` - Implement Initializeddestructuringassignmenttypes (same feature label, same group key, title overlap)
- `issues/open/3097-implement-jsFileCompilationTypeArgumentSyntaxOfCall.md` - Implement Jsfilecompilationtypeargumentsyntaxofcall (same feature label, same group key, title overlap)
- `issues/open/3125-implement-jsxEmitWithAttributes.md` - Implement Jsxemitwithattributes (same feature label, same group key, title overlap)
- `issues/open/3126-implement-jsxFactoryAndReactNamespace.md` - Implement Jsxfactoryandreactnamespace (same feature label, same group key, title overlap)
- `issues/open/3127-implement-jsxFactoryIdentifier.md` - Implement Jsxfactoryidentifier (same feature label, same group key, title overlap)
- `issues/open/3130-implement-jsxFactoryMissingErrorInsideAClass.md` - Implement Jsxfactorymissingerrorinsideaclass (same feature label, same group key, title overlap)
- `issues/open/3131-implement-jsxFactoryNotIdentifierOrQualifiedName.md` - Implement Jsxfactorynotidentifierorqualifiedname (same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage regexp literal: RegExp decimal escape class range

- Issue class: `triage-needed`
- Feature label: `regexp-literal`
- Diagnostic: `UnsupportedRegExp` / `unsupported-feature-boundary`
- Path: `reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/built-ins/RegExp/RegExp-decimal-escape-class-range.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 998,
  "lines": 29,
  "extension": ".js",
  "first_code_line": "info: |",
  "test262_metadata": {
    "info": "|",
    "The production CharacterClass": ": [ [lookahead \\notin {^}] ClassRanges ]",
    "es5id": "15.10.2.13_A1_T16",
    "es6id": "B.1.4",
    "description": ">"
  }
}
```

Failure location:

```json
{
  "code": "UnsupportedRegExp",
  "message": "issue-051: RegExp.prototype.exec literal `/[\\d][\\12-\\14]{1,}[^\\d]/` is not supported yet: only plain literal byte patterns and . \\d \\w \\s + * ? are supported",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "regexp-literal",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text

function print(message) {
  console.log(message);
}


/* standard globals shim */
```

Visible symbols before failure:

```json
[
  {
    "kind": "function",
    "name": "print",
    "line": 2,
    "column": 1,
    "params": "message"
  },
  {
    "kind": "binding",
    "name": "NaN",
    "line": 10,
    "column": 1,
    "initializer": "0/0"
  },
  {
    "kind": "binding",
    "name": "Infinity",
    "line": 11,
    "column": 1,
    "initializer": "1/0"
  },
  {
    "kind": "binding",
    "name": "$262",
    "line": 17,
    "column": 1,
    "initializer": "{}"
  },
  {
    "kind": "function",
    "name": "$ERROR",
    "line": 26,
    "column": 1,
    "params": "message"
  },
  {
    "kind": "function",
    "name": "$DONOTEVALUATE",
    "line": 30,
    "column": 1,
    "params": ""
  },
  {
    "kind": "function",
    "name": "assert",
    "line": 34,
    "column": 1,
    "params": "mustBeTrue, message"
  },
  {
    "kind": "binding",
    "name": "__executed",
    "line": 59,
    "column": 1,
    "initializer": "/[\\d][\\12-\\14]{1,}[^\\d]/.exec(\"line1\\n\\n\\n\\n\\nline2\")"
  },
  {
    "kind": "binding",
    "name": "__expected",
    "line": 61,
    "column": 1,
    "initializer": "[\"1\\n\\n\\n\\n\\nl\"]"
  },
  {
    "kind": "binding",
    "name": "index",
    "line": 70,
    "column": 5,
    "initializer": "0"
  }
]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/open/066-implement-regexp-literal.md",
    "title": "Implement RegExp literal support",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/3777-implement-parseJsxElementInUnaryExpressionNoCrash-regexp-literal.md",
    "title": "Implement Parsejsxelementinunaryexpressionnocrash Regexp Literal",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/421-implement-class.md",
    "title": "Implement class syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/4697-implement-unusedImports-regexp-literal.md",
    "title": "Implement Unusedimports Regexp Literal",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/4806-implement-class.md",
    "title": "Implement class syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/051-implement-regexp.md",
    "title": "Implement RegExp",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/444-implement-regexp-literal.md",
    "title": "Implement RegExp literal support",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/4812-implement-regexp-literal.md",
    "title": "Implement RegExp literal support",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Create a child issue around this exact path and diagnostic before broadening the reference window.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Function,
        span: Span {
            start: 1,
            end: 9,
        },
    },
    SpannedToken {
        kind: Ident(
            "print",
        ),
        span: Span {
            start: 10,
            end: 15,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 15,
            end: 16,
        },
    },
    SpannedToken {
        kind: Ident(
            "message",
        ),
        span: Span {
            start: 16,
            end: 23,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 23,
            end: 24,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 25,
            end: 26,
        },
    },
    SpannedToken {
        kind: Ident(
            "console",
        ),
        span: Span {
            start: 29,
            end: 36,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 36,
            end: 37,
        },
    },
    SpannedToken {
        kind: Ident(
            "log",
        ),
        span: Span {
            start: 37,
            end: 40,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 40,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "message",
        ),
        span: Span {
            start: 41,
            end: 48,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 48,
            end: 49,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 49,
            end: 50,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 51,
            end: 52,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 85,
            end: 88,
        },
    },
    SpannedToken {
        kind: Ident(
            "NaN",
        ),
        span: Span {
            start: 89,
            end: 92,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 93,
            end: 94,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 95,
            end: 96,
        },
    },
    SpannedToken {
        kind: Slash,
        span: Span {
            start: 96,
            end: 97,
        },
    },
    SpannedToken {
        kind: Number(
            0,
        ),
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 98,
            end: 99,
        },
    },
    SpannedToken {
        kind: Var,
        span: Span {
            start: 100,
            end: 103,
        },
    },
    SpannedToken {
        kind: Ident(
            "Infinity",
        ),
        span: Span {
            start: 104,
            end: 112,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 113,
            end: 114,
        },
    },
    SpannedToken {
        kind: Number(
            1,
        ),
        span: Span {
            start: 115,
            end: 116,
        },
    },
    SpannedToken {
        kind: Slash,
        span: Span {
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Function {
        name: "print",
        params: [
            (
                "message",
                None,
                false,
            ),
        ],
        body: [
            Expr {
                expr: Call {
                    callee: Member {
                        object: Ident {
                            name: "console",
                            span: Span {
                                start: 29,
                                end: 36,
                            },
                        },
                        property: "log",
                        span: Span {
                            start: 29,
                            end: 40,
                        },
                    },
                    args: [
                        Ident {
                            name: "message",
                            span: Span {
                                start: 41,
                                end: 48,
                            },
                        },
                    ],
                    span: Span {
                        start: 29,
                        end: 49,
                    },
                },
                span: Span {
                    start: 29,
                    end: 50,
                },
            },
        ],
        is_generator: false,
        span: Span {
            start: 1,
            end: 50,
        },
    },
    Let {
        name: "NaN",
        expr: Binary {
            left: Number {
                value: 0,
                span: Span {
                    start: 95,
                    end: 96,
                },
            },
            op: Divide,
            right: Number {
                value: 0,
                span: Span {
                    start: 97,
                    end: 98,
                },
            },
            span: Span {
                start: 95,
                end: 98,
            },
        },
        span: Span {
            start: 85,
            end: 99,
        },
    },
    Let {
        name: "Infinity",
        expr: Binary {
            left: Number {
                value: 1,
                span: Span {
                    start: 115,
                    end: 116,
                },
            },
            op: Divide,
            right: Number {
                value: 0,
                span: Span {
                    start: 117,
                    end: 118,
                },
            },
            span: Span {
                start: 115,
                end: 118,
            },
        },
        span: Span {
            start: 100,
            end: 119,
        },
    },
    Let {
        name: "$262",
        expr: Object {
            props: [],
            span: Span {
                start: 182,
                end: 184,
            },
        },
        span: Span {
            start: 171,
            end: 185,
        },
    },
    Expr {
        expr: PropertyAssign {
            object: Ident {
                name: "$262",
                span: Span {
                    start: 186,
                    end: 190,
                },
            },
            property: "gc",
            value: FunctionExpr {
                name: "",
                params: [],
                body: [],
                span: Span {
                    start: 196,
                    end: 204,
                },
            },
            span: Span {
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedRegExp] issue-051: RegExp.prototype.exec literal `/[\d][\12-\14]{1,}[^\d]/` is not supported yet: only plain literal byte patterns and . \d \w \s + * ? are supported
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
        "code": 6504,
        "category": "Error",
        "message": "File '/tmp/tmpoyl6t3jb/test262-triage-node-input.js' is a JavaScript file. Did you mean to enable the 'allowJs' option?\n  The file is in the program because:\n    Root file specified for compilation"
      }
    ],
    "hints": [],
    "typescriptVersion": "6.0.3"
  }
}
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
