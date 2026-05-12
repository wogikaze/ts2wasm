# Coverage Expansion Epics — Requirements

Run: `coverage-epics-2026-05-12`
Created: 2026-05-13

## Source

- Epic design: `docs/superpowers/specs/2026-05-12-coverage-epics-design.md`
- Issues: `issues/I-20260513-*` (6 epic issues)

## Requirements

6 independent coverage expansion epics, each implemented in a separate child worktree:

| # | Issue | Priority | Area | Target |
|---|-------|----------|------|--------|
| 1 | I-20260513-BRRFMC | P1 | Builtin API | semantic +100 |
| 2 | I-20260513-9SQA5S | P1 | Class | unsupported -6609 |
| 3 | I-20260513-4HFDDM | P1 | TS + tsc/tsgo | tsc build >=10% |
| 4 | I-20260513-VWANM5 | P2 | Async/Await | unsupported -1332 |
| 5 | I-20260513-PY42B3 | P2 | Import/Export | unsupported -1138 |
| 6 | I-20260513-9MXNCK | P2 | Name Resolution | unsupported -13426 |

## Dependency Graph

All 6 epics are independent (`depends_on: []`). Parallel execution OK.

## Verification Gate

```bash
mise run gate                           # fmt + arch + matrix + nextest
mise run reference-coverage -- test262 --limit 200 --detail  # sample regression check
```

## Child Worktree Assignment

Each child implements its epic scope, runs focused gate, commits.
Parent merges all branches, resolves conflicts, runs full gate, updates coverage matrix.
