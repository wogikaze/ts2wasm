---
id: 562
title: "Implement Accessinstancememberfromstaticmethod"
type: spike
area: frontend/resolver
class: done
priority: P1
depends_on: [5391]
blocks: []
created: 2026-05-01
updated: 2026-05-07
completed: 2026-05-07
---

## Summary

Close `accessInstanceMemberFromStaticMethod` after splitting the current
unqualified static-member diagnostic blocker into issue 5391.

## Problem

Reference test results originally showed 1 case failing in directory
`accessInstanceMemberFromStaticMethod` with diagnostics: name-resolution.
Fresh triage on 2026-05-07 confirms the current failure is the focused
unqualified `foo` diagnostic gap now owned by issue 5391.

Problem: `accessInstanceMemberFromStaticMethod01.ts` currently reports generic
`UnresolvedName: foo` where TypeScript reports TS2662 with a `C.foo` static
member suggestion.

## Current failure

Representative reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts
```

Coverage window:

```sh
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts --detail
```

## Desired final state

This generated bucket is closed after splitting the exact current blocker into
issue 5391.

## Scope

In scope:

- [x] Inspect the smart triage report below
- [x] Confirm whether existing open/done issues already cover this bucket
- [x] Split one feature family, one observable behavior, or one fixed reference window into child issues
- [x] Preserve exact reproduction commands and representative AST/diagnostic evidence in issue 5391

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

- [x] Duplicate candidates below are confirmed as no-match for the exact current failure
- [x] Child issue 5391 contains exact `reference-triage` commands
- [x] Child issue 5391 includes failing path, diagnostic code, source context, visible symbols, and parser/TypeScript AST evidence
- [x] Child issue 5391 acceptance names the exact reference path and diagnostic/stdout change

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
```

Impacted commands:

```sh
mise run reference-coverage -- tsc --limit 2
mise run reference-coverage -- tsc --path-filter reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts --detail
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts
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

- [x] created: `issues/open/5391-report-unqualified-class-member-name-diagnostics.md`

## Notes

## Affected test files

- `reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts`

## Duplicate detection

- `issues/open/091-implement-accessInstanceMemberFromStaticMethod.md` - Implement Accessinstancememberfromstaticmethod (same reference path, same group key, title overlap)
- `issues/open/476-implement-accessInstanceMemberFromStaticMethod.md` - Implement Accessinstancememberfromstaticmethod (same reference path, same feature label, same group key, title overlap)

## Smart triage

Generated 2026-05-07.

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts

result:
UnresolvedName / name-resolution

current diagnostic:
unresolved name: `foo`

source:
class C {
    static foo: string;
    bar() {
        let k = foo;
    }
}

compiler evidence:
tokens: ok
ast: ok
resolved/lowered: fails with UnresolvedName `foo`

TypeScript oracle:
TS2662 at `foo`: "Cannot find name 'foo'. Did you mean the static member
'C.foo'?"

decision:
split to issues/open/5391-report-unqualified-class-member-name-diagnostics.md
```

## Historical smart triage

### Smart triage: Triage name resolution: accessInstanceMemberFromStaticMethod01

- Issue class: `triage-needed`
- Feature label: `name-resolution`
- Diagnostic: `UnresolvedName` / `resolver-symbol`
- Path: `reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts`

Reproduction:

```sh
mise run reference-triage -- tsc reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts
```

Source overview:

```json
{
  "suite": "tsc",
  "bytes": 94,
  "lines": 8,
  "extension": ".ts",
  "first_code_line": "class C {"
}
```

Failure location:

```json
{
  "code": "UnresolvedName",
  "message": "unresolved name: `foo`",
  "span_start": null,
  "span_end": null,
  "line": null,
  "column": null,
  "feature_label": "name-resolution",
  "error_type": "resolver-symbol"
}
```

Source context:

```text
// @target: es2015
class C {
    static foo: string;

    bar() {
        let k = foo;
    }
}
```

Visible symbols before failure:

```json
[
  {
    "kind": "class",
    "name": "C",
    "line": 2,
    "column": 1
  },
  {
    "kind": "binding",
    "name": "k",
    "line": 6,
    "column": 9,
    "initializer": "foo"
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
    "path": "issues/open/091-implement-accessInstanceMemberFromStaticMethod.md",
    "title": "Implement Accessinstancememberfromstaticmethod",
    "reason": "same reference path"
  },
  {
    "state": "open",
    "path": "issues/open/437-implement-name-resolution.md",
    "title": "Implement name resolution",
    "reason": "same feature label, title overlap"
  },
  {
    "state": "open",
    "path": "issues/open/476-implement-accessInstanceMemberFromStaticMethod.md",
    "title": "Implement Accessinstancememberfromstaticmethod",
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
        kind: Class,
        span: Span {
            start: 20,
            end: 25,
        },
    },
    SpannedToken {
        kind: Ident(
            "C",
        ),
        span: Span {
            start: 26,
            end: 27,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 28,
            end: 29,
        },
    },
    SpannedToken {
        kind: Static,
        span: Span {
            start: 35,
            end: 41,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 42,
            end: 45,
        },
    },
    SpannedToken {
        kind: Colon,
        span: Span {
            start: 45,
            end: 46,
        },
    },
    SpannedToken {
        kind: Ident(
            "string",
        ),
        span: Span {
            start: 47,
            end: 53,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 53,
            end: 54,
        },
    },
    SpannedToken {
        kind: Ident(
            "bar",
        ),
        span: Span {
            start: 62,
            end: 65,
        },
    },
    SpannedToken {
        kind: LeftParen,
        span: Span {
            start: 65,
            end: 66,
        },
    },
    SpannedToken {
        kind: RightParen,
        span: Span {
            start: 66,
            end: 67,
        },
    },
    SpannedToken {
        kind: LeftBrace,
        span: Span {
            start: 68,
            end: 69,
        },
    },
    SpannedToken {
        kind: Let,
        span: Span {
            start: 79,
            end: 82,
        },
    },
    SpannedToken {
        kind: Ident(
            "k",
        ),
        span: Span {
            start: 83,
            end: 84,
        },
    },
    SpannedToken {
        kind: Equal,
        span: Span {
            start: 85,
            end: 86,
        },
    },
    SpannedToken {
        kind: Ident(
            "foo",
        ),
        span: Span {
            start: 87,
            end: 90,
        },
    },
    SpannedToken {
        kind: Semicolon,
        span: Span {
            start: 90,
            end: 91,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 97,
            end: 98,
        },
    },
    SpannedToken {
        kind: RightBrace,
        span: Span {
            start: 100,
            end: 101,
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
    ClassDecl {
        name: "C",
        extends: None,
        body: [
            Function {
                name: "bar",
                params: [],
                body: [
                    Let {
                        name: "k",
                        expr: Ident {
                            name: "foo",
                            span: Span {
                                start: 87,
                                end: 90,
                            },
                        },
                        span: Span {
                            start: 79,
                            end: 91,
                        },
                    },
                ],
                is_generator: false,
                span: Span {
                    start: 62,
                    end: 91,
                },
            },
        ],
        static_blocks: [],
        private_elements: [],
        span: Span {
            start: 20,
            end: 101,
        },
    },
]
```

#### resolved

- ok: `False`
- truncated: `False`

```text
error: [UnresolvedName] unresolved name: `foo`
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
        "code": 2662,
        "category": "Error",
        "message": "Cannot find name 'foo'. Did you mean the static member 'C.foo'?",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts",
        "start": 87,
        "length": 3,
        "line": 6,
        "character": 17
      }
    ],
    "hints": [
      {
        "kind": "binding",
        "typeText": "any",
        "file": "/home/wogikaze/wgkz/ts2wasm/reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts",
        "start": 83,
        "length": 1,
        "line": 6,
        "character": 13,
        "name": "k"
      }
    ],
    "typescriptVersion": "6.0.3"
  }
}
```

## Completion evidence

Commits:

- Split current unqualified static-member diagnostic blocker to issue 5391; no
  direct implementation from this generated bucket.

Validation result:

```text
command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-coverage tsc --path-filter reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts --detail --no-dashboard-data
result:
executed=1, build_pass=0, unsupported=1, unsupported_diagcodes=UnresolvedName:1
date:
2026-05-07

command:
env TS2WASM_BINARY=/tmp/ts2wasm-issue-blockers-target/debug/ts2wasm python scripts/manager.py reference-triage tsc reference/typescript/tests/cases/compiler/accessInstanceMemberFromStaticMethod01.ts
result:
UnresolvedName / name-resolution; current diagnostic is `unresolved name: foo`; TypeScript oracle reports TS2662 with `C.foo` suggestion
date:
2026-05-07
```

Remaining risks:

- Issue 5391 owns implementation and follow-up verification.

## False-done audit

**truly-done** (562)

- Implementation commits: verified via `git log --oneline --all --grep=562`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
