---
id: 316
title: "Implement enum support"
type: spike
area: frontend/syntax
class: triage-needed
priority: P1
depends_on: []
blocks: []
created: 2026-04-30
updated: 2026-04-30
---

## Summary

Triage enum feature across 8 failing reference test cases and split this bucket into implementation-ready child issues.

## Problem

Reference test results show 8 cases fail with enum diagnostic. The compiler cannot handle these syntax/semantics, preventing compilation of code in this category.

Problem: enum feature has 8 reference failures and needs smart-triage evidence before implementation starts.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js
```

Coverage window:

```sh
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js --detail
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
mise run reference-coverage -- test262 --limit 16
mise run reference-coverage -- test262 --path-filter reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js --detail
mise run reference-triage -- test262 reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js
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

- `reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-else-decl-a-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-else-decl-b-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-else-stmt-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-decl-no-else-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/if-stmt-else-decl-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/switch-case-global-existing-non-enumerable-global-init.js`
- `reference/test262/test/annexB/language/global-code/switch-dflt-global-existing-non-enumerable-global-init.js`

## Duplicate detection

- `issues/open/144-implement-ambientConstLiterals.md` - Implement Ambientconstliterals (same feature label, same group key, title overlap)
- `issues/open/145-implement-ambientEnum.md` - Implement Ambientenum (same feature label, same group key, title overlap)
- `issues/open/146-implement-ambientEnumElementInitializer.md` - Implement Ambientenumelementinitializer (same feature label, same group key, title overlap)
- `issues/open/159-implement-ambientModuleWithTemplateLiterals.md` - Implement Ambientmodulewithtemplateliterals (same feature label, same group key, title overlap)
- `issues/open/175-implement-amdModuleConstEnumUsage.md` - Implement Amdmoduleconstenumusage (same feature label, same group key, title overlap)
- `issues/open/255-implement-private-class-element-runtime-semantics.md` - Implement private class element runtime semantics (same feature label, same group key, title overlap)
- `issues/open/269-implement-math-pow.md` - Implement Math.pow (same feature label, same group key, title overlap)
- `issues/open/270-implement-array-prototype-map.md` - Implement Array.prototype.map (same feature label, same group key, title overlap)
- `issues/open/274-implement-spread-operator.md` - Implement spread operator (same feature label, same group key, title overlap)
- `issues/done/020a-design-javascript-semantic-ir.md` - issues/done/020a-design-javascript-semantic-ir.md (same feature label, same group key)

## Smart triage

### Smart triage: Triage name resolution: block decl global existing non enumerable global init

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js`

Reproduction:

```sh
mise run reference-triage -- test262 reference/test262/test/annexB/language/global-code/block-decl-global-existing-non-enumerable-global-init.js
```

Source overview:

```json
{
  "suite": "test262",
  "bytes": 1278,
  "lines": 49,
  "extension": ".js",
  "first_code_line": "description: Variable binding is left in place by legacy function hoisting. CreateGlobalVariableBinding leaves the binding as non-enumerable even if it has the ",
  "test262_metadata": {
    "description": "Variable binding is left in place by legacy function hoisting. CreateGlobalVariableBinding leaves the binding as non-enumerable even if it has the chance to change it to be enumerable. (Block statement in the global scope containing a function declaration)",
    "esid": "sec-web-compat-globaldeclarationinstantiation",
    "flags": "[generated, noStrict]",
    "includes": "[fnGlobalObject.js, propertyHelper.js]",
    "info": "|"
  }
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `fnGlobalObject` at 746..760",
  "span_start": 746,
  "span_end": 760,
  "line": 17,
  "column": 14,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
14 |     [...]
15 | 
16 | ---*/
17 | var global = fnGlobalObject();
18 | Object.defineProperty(global, 'f', {
19 |   value: 'x',
20 |   enumerable: false,
```

Visible symbols before failure:

```json
[
  {
    "kind": "binding",
    "name": "global",
    "line": 17,
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
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/291-provide-object-global-binding-for-test262.md",
    "title": "Provide Object global binding for test262 cases",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "done",
    "path": "issues/done/302-implement-direct-eval-block-function-declaration-slice.md",
    "title": "Implement direct eval block function declaration slice",
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
        kind: Var,
        span: Span {
            start: 733,
            end: 736,
        },
    },
    SpannedToken {
        kind: Ident(
            "global",
        ),
        span: Span {
            start: 737,
            end: 743,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 744,
            end: 745,
        },
    },
    SpannedToken {
        kind: Ident(
            "fnGlobalObject",
        ),
        span: Span {
            start: 746,
            end: 760,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 760,
            end: 761,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 761,
            end: 762,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 762,
            end: 763,
        },
    },
    SpannedToken {
        kind: Ident(
            "Object",
        ),
        span: Span {
            start: 764,
            end: 770,
        },
    },
    SpannedToken {
        kind: Dot,
        span: Span {
            start: 770,
            end: 771,
        },
    },
    SpannedToken {
        kind: Ident(
            "defineProperty",
        ),
        span: Span {
            start: 771,
            end: 785,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 785,
            end: 786,
        },
    },
    SpannedToken {
        kind: Ident(
            "global",
        ),
        span: Span {
            start: 786,
            end: 792,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 792,
            end: 793,
        },
    },
    SpannedToken {
        kind: String(
            "f",
        ),
        span: Span {
            start: 794,
            end: 797,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 797,
            end: 798,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 799,
            end: 800,
        },
    },
    SpannedToken {
        kind: Ident(
            "value",
        ),
        span: Span {
            start: 803,
            end: 808,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 808,
            end: 809,
        },
    },
    SpannedToken {
        kind: String(
            "x",
        ),
        span: Span {
            start: 810,
            end: 813,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 813,
            end: 814,
        },
    },
    SpannedToken {
        kind: Ident(
            "enumerable",
        ),
        span: Span {
            start: 817,
            end: 827,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 827,
            end: 828,
        },
    },
    SpannedToken {
        kind: False,
        span: Span {
            start: 829,
            end: 834,
        },
    },
    SpannedToken {
        kind: Comma,
        span: Span {
            start: 834,
            end: 835,
        },
    },
    SpannedToken {
        kind: Ident(
            "writable",
        ),
        span: Span {
            start: 838,
            end: 846,
        },
    },
```

#### ast

- ok: `True`
- truncated: `True`

```text
== ast ==
[
    Let {
        name: "global",
        expr: Call {
            callee: Ident {
                name: "fnGlobalObject",
                span: Span {
                    start: 746,
                    end: 760,
                },
            },
            args: [],
            span: Span {
                start: 746,
                end: 762,
            },
        },
        span: Span {
            start: 733,
            end: 763,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "Object",
                    span: Span {
                        start: 764,
                        end: 770,
                    },
                },
                property: "defineProperty",
                span: Span {
                    start: 764,
                    end: 785,
                },
            },
            args: [
                Ident {
                    name: "global",
                    span: Span {
                        start: 786,
                        end: 792,
                    },
                },
                String {
                    value: "f",
                    span: Span {
                        start: 794,
                        end: 797,
                    },
                },
                Object {
                    props: [
                        (
                            "value",
                            String {
                                value: "x",
                                span: Span {
                                    start: 810,
                                    end: 813,
                                },
                            },
                        ),
                        (
                            "enumerable",
                            Bool {
                                value: false,
                                span: Span {
                                    start: 829,
                                    end: 834,
                                },
                            },
                        ),
                        (
                            "writable",
                            Bool {
                                value: true,
                                span: Span {
                                    start: 848,
                                    end: 852,
                                },
                            },
                        ),
                        (
                            "configurable",
                            Bool {
                                value: true,
                                span: Span {
                                    start: 870,
                                    end: 874,
                                },
                            },
                        ),
                    ],
                    span: Span {
                        start: 799,
                        end: 876,
                    },
                },
            ],
            span: Span {
                start: 764,
                end: 877,
            },
        },
        span: Span {
            start: 764,
            end: 878,
        },
    },
    Expr {
        expr: Call {
            callee: Member {
                object: Ident {
                    name: "$262",
                    span: Span {
                        start: 880,
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `fnGlobalObject` at 746..760
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
error: [UnresolvedName] unresolved name: `fnGlobalObject` at 746..760
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
