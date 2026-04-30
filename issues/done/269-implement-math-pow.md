---
id: 269
title: "Implement integer-only Math.pow slice"
type: feature
area: runtime/builtins
class: done
priority: P2
tracking: feature:math-builtins
updated: 2026-04-30
completed: 2026-04-30
---

## Summary

Close this issue for the already implemented `Math.pow` runtime slice:
non-negative integer exponentiation over the current integer-backed number
subset.

This issue does not claim full ECMAScript `Math.pow` compatibility. The
floating-point, `NaN`, `Infinity`, `-0`, negative-exponent, and broad Test262
semantics are split to issue 317.

## Evidence

AtCoder ABC451 D uses integer powers of two:

```typescript
const result = Math.pow(2, n);
```

The implemented slice added:

- `BuiltinId::MathPow`
- `Math.pow` builtin resolution
- `RuntimeFn::MathPow`
- `$math_pow` runtime emission
- dump-name support for `Math.pow`

The runtime helper is intentionally integer-only for the current number model.
It handles numeric type checks, exponent `0`, rejects negative exponents by
returning `undefined`, and computes non-negative integer powers by repeated
multiplication.

## Acceptance criteria

1. [x] `Math` object is available in global scope for the supported builtin path.
2. [x] `Math.pow` resolves and builds for integer-backed number arguments.
3. [x] Typical integer use cases such as `Math.pow(2, 3)` return the expected value.
4. [x] The unsupported full-number semantics are not hidden in this closed issue.
5. [x] Full `Math.pow` compatibility is tracked by a separate open issue.

## Validation

Required commands:

```bash
cargo fmt --all --check
cargo nextest run -E 'test(math) or test(array_map) or test(node_diff)'
python scripts/manager.py update-issue-index --check
python scripts/manager.py check issues
```

## Reopened by audit

Date: 2026-04-30

Classification: false-done / incomplete acceptance.

Reason: the issue was under `issues/done/` while its acceptance and own
limitations still claimed unsupported `Math.pow` behavior: Infinity, NaN, +0,
-0, floating-point semantics, and Test262 coverage remained incomplete. Issue
296 deliberately closed only the `**` small-int operator slice and explicitly
left full `Math.pow` compatibility out of scope.

Next close bar resolution:

- This issue is narrowed to the implemented integer-only `Math.pow` slice.
- Residual full `Math.pow` number compatibility is split to
  `issues/open/317-implement-full-math-pow-number-semantics.md`.

## Completion evidence

Commits:

- `d01ffb2`: Implement Math.pow builtin function.
- close commit on branch `agent/269-270-math-pow-array-map-20260430T000000Z`.

Validation result:

```text
command: cargo fmt --all --check
result: pass
date: 2026-04-30

command: cargo nextest run -E 'test(math) or test(array_map) or test(node_diff)'
result: fail after issue edits while compiling unrelated split test modules:
  crates/frontend/src/lexer_tests.rs missing direct imports for Diagnostic/Lexer/Token/DiagCode
  crates/cli/tests/m2_node_diff_fixture_tests.rs compiled as a standalone integration test and missing parent module helpers
date: 2026-04-30

command: python scripts/manager.py update-issue-index --check
result: pass after issue edits
date: 2026-04-30

command: python scripts/manager.py check issues
result: pass after issue edits after restoring ignored local artifact
  artifacts/coverage/results/test262-results.jsonl from the parent worktree
date: 2026-04-30

command: python scripts/manager.py check agent-state
result: pass
date: 2026-04-30
```

Remaining risks:

- Full ECMAScript `Math.pow` compatibility remains open in issue 317.
