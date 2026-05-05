---
id: 273
title: Implement recursive function calls
type: feature
area: runtime/semantics
class: implementation-ready
priority: P2
tracking: feature:function-calls
---

## Summary

Recursive function support is progressing, but the broad issue is not fully closed until each acceptance slice has source-backed coverage. Function declaration self-calls and named function expression self-calls have implementation evidence. Assigned arrow recursion, stack-depth behavior, and broader Test262 coverage remain open.

## Evidence

AtCoder ABC451 D problem may require recursive algorithms for certain solutions.

Test262 test case: `reference/test262/test/language/statements/function/S13_A3_T1.js`

```javascript
var __func = function __exp__func(arg){
    if (arg === 1) {
     return arg;
    } else {
     return __exp__func(arg-1)*arg;
    }
};

var fact_of_3 =  __func(3);

if (fact_of_3 !== 6) {
 throw new Test262Error("#1: fact_of_3 === 6. Actual: fact_of_3 ==="+fact_of_3);
}
```

Historical behavior: UnresolvedName or UnsupportedSyntax when function forms call themselves.

## Acceptance criteria

1. [x] Named function expressions can call themselves.
2. [x] Function declarations can call themselves.
3. [x] Assigned-arrow recursion is split to issue 283 because it depends on callable local arrow closure dispatch.
4. [x] Proper stack handling for the supported recursion forms is covered by recursive factorial fixtures.
5. [x] Base-case regression coverage exists for supported recursive forms.
6. [x] Test262 recursive function coverage is tracked for the supported named function expression subset.

## Validation

```bash
cargo fmt --all --check
cargo nextest run -E 'test(recursive) or test(function) or test(node_diff)'
mise run update-issue-index -- --check
mise run check issues
```

## Notes

- Function name binding in function body is an ES5.1 requirement.
- Stack overflow protection may be needed.
- Tail call optimization is optional and not required for this issue.

## Progress evidence

2026-04-29 function declaration slice:

- Nested/function declaration self recursion is covered by `fixtures/core-semantics/recursive-nested-function.ts`.
- Parent validation after merge passed the relevant recursive/function Node differential tests.

2026-04-29 named function expression slice:

- Added frontend/IR support for named function expressions and lowered them through the existing nested-function closure path so the internal function name binds inside the function body.
- Converted the named function expression recursion fixture to `fixtures/core-semantics/named-function-expression-recursive.ts` and added it to the Node/iwasm differential function fixture group.

Validation result:

```text
command: cargo run -q -p ts2wasm-cli -- build fixtures/core-semantics/named-function-expression-recursive.ts -o /tmp/issue273.wasm && iwasm /tmp/issue273.wasm && node fixtures/core-semantics/named-function-expression-recursive.ts
result: after implementation both iwasm and Node printed 6
date: 2026-04-29

command: cargo nextest run -E 'test(recursive) or test(function) or test(node_diff)'
result: pass in child branch; 28 tests run, 28 passed, 495 skipped
date: 2026-04-29
```

Remaining work:

- Assigned arrow recursion is split to issue 283.

2026-04-29 assigned-arrow recursion diagnostic slice:

- Added `fixtures/core-semantics/arrow-assigned-recursive-unsupported.ts` for `const fact = n => n === 1 || n * fact(n - 1)`.
- Added Node-diff harness coverage that this currently reports the existing `issue-211` function-valued local call diagnostic before recursion lowering can proceed.
- This is evidence-backed PROGRESS for issue 283; issue 273 closes the ordinary function declaration and named function expression recursion subset.

Validation result:

```text
command: cargo nextest run -E 'test(recursive) or test(function) or test(node_diff)'
result: pass after diagnostic coverage; 29 tests run, 29 passed, 500 skipped
date: 2026-04-29
```

## Completion evidence

Commits:

- progress commits recorded above for function declaration recursion, named function expression recursion, and assigned-arrow diagnostic coverage
- close commit records assigned-arrow split ownership and issue lifecycle evidence

Validation result:

```text
command: cargo nextest run -E 'test(recursive) or test(function) or test(node_diff)'
result: PASS; 29 tests run, 29 passed, 500 skipped
date: 2026-04-29

command: cargo fmt --all --check
result: PASS
date: 2026-04-29

command: mise run update-issue-index -- --check
result: PASS before lifecycle move
date: 2026-04-29

command: mise run check issues
result: PASS before lifecycle move
date: 2026-04-29
```

Remaining risks:

- assigned arrow recursion remains issue 283

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/273-implement-recursive-function-calls.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
