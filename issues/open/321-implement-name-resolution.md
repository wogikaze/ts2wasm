---
id: 321
title: "Implement name resolution"
type: spike
area: frontend/resolver
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Triage name-resolution feature across 3 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 3 cases fail with name-resolution diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: name-resolution feature has 3 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/comments/single-line-html-close-first-line-1.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/comments/single-line-html-close-first-line-1.js --detail
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
mise run reference-coverage -- test262 --limit 6
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/comments/single-line-html-close-first-line-1.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/comments/single-line-html-close-first-line-1.js
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

- `reference/test262/test/annexB/language/comments/single-line-html-close-first-line-1.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-first-line-2.js`
- `reference/test262/test/annexB/language/comments/single-line-html-close-first-line-3.js`

## Duplicate detection

- `issues/open/064-implement-name-resolution.md` - Implement name resolution (same feature label, same group key, title overlap)
- `issues/open/089-implement-acceptSymbolAsWeakType.md` - Implement Acceptsymbolasweaktype (same feature label, same group key, title overlap)
- `issues/open/193-implement-arguments.md` - Implement Arguments (same feature label, same group key, title overlap)
- `issues/open/195-implement-argumentsBindsToFunctionScopeArgumentList.md` - Implement Argumentsbindstofunctionscopeargumentlist (same feature label, same group key, title overlap)
- `issues/open/196-implement-argumentsObjectCreatesRestForJs.md` - Implement Argumentsobjectcreatesrestforjs (same feature label, same group key, title overlap)
- `issues/open/225-implement-eval-annexb-function-declarations.md` - Implement eval and Annex B function declaration semantics (same feature label, same group key, title overlap)
- `issues/done/005-add-fine-grained-unsupported-feature-breakdown.md` - issues/done/005-add-fine-grained-unsupported-feature-breakdown.md (same feature label, same group key)
- `issues/done/022-expand-test262-differential-coverage.md` - issues/done/022-expand-test262-differential-coverage.md (same feature label, same group key)
- `issues/done/060-investigate-unknown-unsupported-cases.md` - Investigate and classify unknown-unsupported diagnostic cases (same feature label, same group key)
- `issues/done/224-implement-annexb-html-comments.md` - Implement Annex B HTML-like comments (same reference path, same feature label, same group key, title overlap)

## Smart triage

### Smart triage: Triage name resolution: single line html close first line 1

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/test262/test/annexB/language/comments/single-line-html-close-first-line-1.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/comments/single-line-html-close-first-line-1.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1102,
  "lines": 34,
  "extension": ".js",
  "first_code_line": "--> a comment",
  "test262_metadata": {
    "esid": "sec-html-like-comments",
    "description": ">",
    "flags": "[raw]",
    "info": "|",
    "InputElementHashbangOrRegExp": ":",
    "HTMLCloseComment": ":",
    "negative": "",
    "phase": "runtime",
    "type": "EvalError"
  }
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `EvalError` at 1063..1072",
  "span_start": 1063,
  "span_end": 1072,
  "line": 34,
  "column": 11,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
31 | // Express the intended behavior by intentionally throwing an error; this
32 | // guarantees that test runners will only consider the test "passing" if
33 | // executable sequences are correctly interpreted as such.
34 | throw new EvalError("This is not in a comment");
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
    "state": "done",
    "path": "issues/done/224-implement-annexb-html-comments.md",
    "title": "Implement Annex B HTML-like comments",
    "reason": "same reference path, same feature label"
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
        kind: Semicolon,
        span: Span {
            start: 0,
            end: 13,
        },
    },
    SpannedToken {
        kind: Throw,
        span: Span {
            start: 1053,
            end: 1058,
        },
    },
    SpannedToken {
        kind: New,
        span: Span {
            start: 1059,
            end: 1062,
        },
    },
    SpannedToken {
        kind: Ident(
            "EvalError",
        ),
        span: Span {
            start: 1063,
            end: 1072,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 1072,
            end: 1073,
        },
    },
    SpannedToken {
        kind: String(
            "This is not in a comment",
        ),
        span: Span {
            start: 1073,
            end: 1099,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 1099,
            end: 1100,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 1100,
            end: 1101,
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
    Throw {
        expr: New {
            expr: Ident {
                name: "EvalError",
                span: Span {
                    start: 1063,
                    end: 1072,
                },
            },
            args: [
                String {
                    value: "This is not in a comment",
                    span: Span {
                        start: 1073,
                        end: 1099,
                    },
                },
            ],
            span: Span {
                start: 1059,
                end: 1100,
            },
        },
        span: Span {
            start: 1053,
            end: 1101,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `EvalError` at 1063..1072
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
error: [UnresolvedName] unresolved name: `EvalError` at 1063..1072
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
