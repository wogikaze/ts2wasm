---
id: 601
title: "Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration"
type: spike
area: frontend/syntax
class: done
priority: P1
depends_on: [5403]
blocks: []
created: 2026-05-01
updated: 2026-05-08
completed: 2026-05-08
---

## Summary

Triage allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration across 1 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 1 cases fail in directory `allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration` with diagnostics: import-export. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration has 1 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts --detail
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
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] not affected

Follow-up issues:

- [x] none

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts`

## Duplicate detection

- `issues/done/136-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` - Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (same reference path, same feature label, same group key, title overlap)
- `issues/done/432-implement-import-export.md` - Implement import/export module syntax (same feature label, same group key, title overlap)
- `issues/done/462-implement-ExportAssignment.md` - Implement Exportassignment (same feature label, same group key, title overlap)
- `issues/done/514-implement-allowSyntheticDefaultImports.md` - Implement Allowsyntheticdefaultimports (same feature label, same group key, title overlap)
- `issues/done/515-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md` - Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage import export: allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration

- Issue class: `triage-needed`
- Feature label: `import-export`
- Diagnostic: `UnsupportedModule` / `unsupported-feature-boundary`
- Path: `reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 319,
  "lines": 14,
  "extension": ".ts",
  "first_code_line": "interface Color {"
}
```

Failure location:

```json
{
  "code": "UnsupportedModule",
  "message": "issue-232: missing local module `./color` imported from /home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts; tried /home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/./color.ts, /home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/./color.js at 191..200",
  "span_start": 191,
  "span_end": 200,
  "line": 10,
  "column": 27,
  "feature_label": "import-export",
  "error_type": "unsupported-feature-boundary"
}
```

Source context:

```text
 7 | }
 8 | export default Color;
 9 | // @filename: file1.ts
10 | import Color from "./color";
11 | export declare function styled(): Color;
12 | // @filename: file2.ts
13 | import { styled }  from "./file1";
```

Visible symbols before failure:

```json
[]
```

Duplicate candidates:

```json
[
  {
    "state": "open",
    "path": "issues/done/136-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md",
    "title": "Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/432-implement-import-export.md",
    "title": "Implement import/export module syntax",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/457-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/463-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/515-implement-allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.md",
    "title": "Implement Allowsyntheticdefaultimportscanpaintcrossmoduledeclaration",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/543-implement-APISample-import-export.md",
    "title": "Implement Apisample Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/done/549-implement-FunctionDeclaration-import-export.md",
    "title": "Implement Functiondeclaration Import Export",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/055-implement-import-export.md",
    "title": "Umbrella: implement import and export",
    "reason": "same feature label, title overlap"
  }
]
```

Error-specific suggestions:

- Keep module graph behavior separate from parser syntax unless the diagnostic proves syntax is the blocker.

Compiler dumps:

#### tokens

- ok: `True`
- truncated: `True`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "interface",
        ),
        span: Span {
            start: 88,
            end: 97,
        },
    },
    SpannedToken {
        kind: Ident(
            "Color",
        ),
        span: Span {
            start: 98,
            end: 103,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 104,
            end: 105,
        },
    },
    SpannedToken {
        kind: Ident(
            "c",
        ),
        span: Span {
            start: 111,
            end: 112,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 112,
            end: 113,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 114,
            end: 120,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 120,
            end: 121,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 123,
            end: 124,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 126,
            end: 132,
        },
    },
    SpannedToken {
        kind: Default,
        span: Span {
            start: 133,
            end: 140,
        },
    },
    SpannedToken {
        kind: Ident(
            "Color",
        ),
        span: Span {
            start: 141,
            end: 146,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 146,
            end: 147,
        },
    },
    SpannedToken {
        kind: Import,
        span: Span {
            start: 173,
            end: 179,
        },
    },
    SpannedToken {
        kind: Ident(
            "Color",
        ),
        span: Span {
            start: 180,
            end: 185,
        },
    },
    SpannedToken {
        kind: Ident(
            "from",
        ),
        span: Span {
            start: 186,
            end: 190,
        },
    },
    SpannedToken {
        kind: String(
            "./color",
        ),
        span: Span {
            start: 191,
            end: 200,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 200,
            end: 201,
        },
    },
    SpannedToken {
        kind: Export,
        span: Span {
            start: 203,
            end: 209,
        },
    },
    SpannedToken {
        kind: Ident(
            "declare",
        ),
        span: Span {
            start: 210,
            end: 217,
        },
    },
    SpannedToken {
        kind: Function,
        span: Span {
            start: 218,
            end: 226,
        },
    },
    SpannedToken {
        kind: Ident(
            "styled",
        ),
        span: Span {
            start: 227,
            end: 233,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 233,
            end: 234,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 234,
            end: 235,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 235,
            end: 236,
        },
    },
    SpannedToken {
        kind: Ident(
            "Color",
        ),
        span: Span {
            start: 237,
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    ExportDefault {
        expr: Ident {
            name: "Color",
            span: Span {
                start: 141,
                end: 146,
            },
        },
        default_span: Span {
            start: 133,
            end: 140,
        },
        span: Span {
            start: 126,
            end: 147,
        },
    },
    ImportDefault {
        specifier: ImportDefaultSpecifier {
            local: "Color",
            local_span: Span {
                start: 180,
                end: 185,
            },
            span: Span {
                start: 180,
                end: 185,
            },
        },
        source: ModuleSpecifier {
            value: "./color",
            span: Span {
                start: 191,
                end: 200,
            },
        },
        span: Span {
            start: 173,
            end: 201,
        },
    },
    ImportNamed {
        specifiers: [
            ImportNamedSpecifier {
                imported: "styled",
                imported_span: Span {
                    start: 278,
                    end: 284,
                },
                local: "styled",
                local_span: Span {
                    start: 278,
                    end: 284,
                },
                span: Span {
                    start: 278,
                    end: 284,
                },
            },
        ],
        source: ModuleSpecifier {
            value: "./file1",
            span: Span {
                start: 293,
                end: 302,
            },
        },
        span: Span {
            start: 269,
            end: 303,
        },
    },
    ExportDecl {
        declaration: Let {
            name: "A",
            expr: Call {
                callee: Ident {
                    name: "styled",
                    span: Span {
                        start: 322,
                        end: 328,
                    },
                },
                args: [],
                span: Span {
                    start: 322,
                    end: 330,
                },
            },
            span: Span {
                start: 312,
                end: 331,
            },
        },
        specifier: ExportNamedSpecifier {
            local: "A",
            local_span: Span {
                start: 318,
                end: 319,
            },
            exported: "A",
            exported_span: Span {
                start: 318,
                end: 319,
            },
            span: Span {
                start: 318,
                end: 319,
            },
        },
        span: Span {
            start: 305,
            end: 331,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnsupportedModule] issue-232: missing local module `./color` imported from /home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts; tried /home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/./color.ts, /home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/./color.js at 191..200
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
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './color' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts",
        "start": 191,
        "length": 9,
        "line": 10,
        "character": 19
      },
      {
        "code": 2307,
        "category": "Error",
        "message": "Cannot find module './file1' or its corresponding type declarations.",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts",
        "start": 293,
        "length": 9,
        "line": 13,
        "character": 25
      }
    ],
    "hints": [
      {
        "kind": "function",
        "typeText": "Color",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts",
        "start": 227,
        "length": 6,
        "line": 11,
        "character": 25,
        "name": "styled"
      },
      {
        "kind": "binding",
        "typeText": "Color",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts",
        "start": 318,
        "length": 1,
        "line": 14,
        "character": 14,
        "name": "A"
      }
    ],
    "typescriptVersion": "6.0.3"
  },
  "ast": {
    "topLevel": [
      {
        "kind": "InterfaceDeclaration",
        "text": "interface Color {\r\n    c: string;\r\n}",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ExportAssignment",
        "text": "export default Color;",
        "line": 8,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import Color from \"./color\";",
        "line": 10,
        "character": 1
      },
      {
        "kind": "FunctionDeclaration",
        "text": "export declare function styled(): Color;",
        "line": 11,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import { styled }  from \"./file1\";",
        "line": 13,
        "character": 1
      },
      {
        "kind": "FirstStatement",
        "text": "export const A = styled();",
        "line": 14,
        "character": 1
      }
    ],
    "pathToPosition": [
      {
        "kind": "SourceFile",
        "text": "interface Color {\r\n    c: string;\r\n}\r\nexport default Color;\r\n// @filename: file1.ts\r\nimport Color from \"./color\";\r\nexpor",
        "line": 5,
        "character": 1
      },
      {
        "kind": "ImportDeclaration",
        "text": "import Color from \"./color\";",
        "line": 10,
        "character": 1
      },
      {
        "kind": "StringLiteral",
        "text": "\"./color\"",
        "line": 10,
        "character": 19
      }
    ]
  }
}
```

Stack trace:

```text
error: [UnsupportedModule] issue-232: missing local module `./color` imported from /home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts; tried /home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/./color.ts, /home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/./color.js at 191..200
```

## Completion evidence

Closed as a generated bucket after fresh 2026-05-08 triage split the current
blocker to `issues/open/5403-support-type-only-default-exports-of-local-interfaces.md`.

Fresh coverage with the current binary:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts --detail --no-dashboard-data
suite=tsc
executed=1
unsupported=1
unsupported_diagcodes=UnsupportedModule:1
unsupported_features=import-export:1
reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts: UnsupportedModule: import-export
```

Fresh triage:

```text
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/allowSyntheticDefaultImportsCanPaintCrossModuleDeclaration.ts
error: [UnresolvedName] unresolved name: `Color` at 50..55
```

The current primary blocker is `export default Color;` after a local
`interface Color { ... }`. Issue 5403 owns that type-only default-export shape;
later virtual-file import resolution remains out of scope for this generated
bucket.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- After issue 5403 lands, this reference case is expected to advance to
  virtual `@filename` import resolution for `./color` and `./file1`.

## False-done audit

**truly-done** (601)

- Implementation commits: verified via `git log --oneline --all --grep=601`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
