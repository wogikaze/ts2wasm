---
id: 322
title: "Implement negative-parse-syntaxerror support"
type: spike
area: reference/triage
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Triage negative-parse-syntaxerror feature across 7 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 7 cases fail with negative-parse-syntaxerror diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: negative-parse-syntaxerror feature has 7 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js --detail
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

- `issues/open/`
- `scripts/run/reference-triage.py`
- `fixtures/`

Do not touch:

- implementation code until the triage report assigns a concrete frontend/runtime/backend owner

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
mise run reference-coverage -- test262 --limit 14
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js
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

- `reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js`
- `reference/test262/test/annexB/language/statements/for-in/bare-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/const-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/let-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/strict-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/var-arraybindingpattern-initializer.js`
- `reference/test262/test/annexB/language/statements/for-in/var-objectbindingpattern-initializer.js`

## Duplicate detection

- `issues/done/229-implement-legacy-octal-escape-handling.md` - Implement legacy octal escape handling (same reference path, title overlap)
- `issues/done/286-classify-negative-syntax-tests-correctly.md` - Classify expected negative SyntaxError tests correctly (same feature label, same group key)

## Smart triage

### Smart triage: Triage name resolution: legacy octal escape sequence strict

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/expressions/template-literal/legacy-octal-escape-sequence-strict.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 484,
  "lines": 18,
  "extension": ".js",
  "first_code_line": "es6id: 12.2.8",
  "test262_metadata": {
    "es6id": "12.2.8",
    "description": ">",
    "The SV of EscapeSequence": ": HexEscapeSequence is the SV of the",
    "negative": "",
    "phase": "parse",
    "type": "SyntaxError",
    "flags": "[onlyStrict]"
  }
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `$DONOTEVALUATE` at 453..467",
  "span_start": 453,
  "span_end": 467,
  "line": 16,
  "column": 1,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
13 | flags: [onlyStrict]
14 | ---*/
15 | 
16 | $DONOTEVALUATE();
17 | 
18 | `${'\07'}`;
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
    "path": "issues/open/064-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/321-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/229-implement-legacy-octal-escape-handling.md",
    "title": "Implement legacy octal escape handling",
    "reason": "same reference path, same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/241-implement-annex-b-date-legacy-methods.md",
    "title": "Implement Annex B Date legacy methods",
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
- truncated: `False`

```text
== tokens ==
[
    SpannedToken {
        kind: Ident(
            "$DONOTEVALUATE",
        ),
        span: Span {
            start: 453,
            end: 467,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 467,
            end: 468,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 468,
            end: 469,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 469,
            end: 470,
        },
    },
    SpannedToken {
        kind: TemplateLiteral(
            "${'\\07'}",
        ),
        span: Span {
            start: 472,
            end: 482,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 482,
            end: 483,
        },
    },
]
```

#### ast

- ok: `True`
- truncated: `False`

```text
== ast ==
[
    Expr {
        expr: Call {
            callee: Ident {
                name: "$DONOTEVALUATE",
                span: Span {
                    start: 453,
                    end: 467,
                },
            },
            args: [],
            span: Span {
                start: 453,
                end: 469,
            },
        },
        span: Span {
            start: 453,
            end: 470,
        },
    },
    Expr {
        expr: Binary {
            left: String {
                value: "",
                span: Span {
                    start: 472,
                    end: 482,
                },
            },
            op: Add,
            right: String {
                value: "\u{7}",
                span: Span {
                    start: 0,
                    end: 5,
                },
            },
            span: Span {
                start: 472,
                end: 482,
            },
        },
        span: Span {
            start: 472,
            end: 483,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `$DONOTEVALUATE` at 453..467
```

TypeScript/JavaScript oracle:

```json
{
  "ok": false,
  "returncode": 2,
  "typescript": {
    "ok": false,
    "error": "failed to load TypeScript compiler API: Cannot find module 'typescript'\nRequire stack:\n- /home/wogikaze/ts2wasm/scripts/check/typescript-oracle.js",
    "diagnostics": [],
    "hints": []
  },
  "ast_error": "node:internal/modules/cjs/loader:1423\n  throw err;\n  ^\n\nError: Cannot find module 'typescript'\nRequire stack:\n- /home/wogikaze/ts2wasm/[eval]\n    at Module._resolveFilename (node:internal/modules/cjs/loader:1420:15)\n    at defaultResolveImpl (node:internal/modules/cjs/loader:1058:19)\n    at resolveForCJSWithHooks (node:internal/modules/cjs/loader:1063:22)\n    at Module._load (node:internal/modules/cjs/loader:1226:37)\n    at TracingChannel.traceSync (node:diagnostics_channel:328:14)\n    at wrapModuleLoad (node:internal/modules/cjs/loader:245:24)\n    at Module.require (node:internal/modules/cjs/loader:1503:12)\n    at require (node:internal/modules/helpers:152:16)\n    at [eval]:3:12\n    at runScriptInThisContext (node:internal/vm:219:10) {\n  code: 'MODULE_NOT_FOUND',\n  requireStack: [ '/home/wogikaze/ts2wasm/[eval]' ]\n}\n\nNode.js v25.2.1\n"
}
```

Stack trace:

```text
error: [UnresolvedName] unresolved name: `$DONOTEVALUATE` at 453..467
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
