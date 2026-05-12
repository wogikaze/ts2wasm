# Coverage Expansion Epics

This document defines the 2026-05-12 coverage expansion wave. It is a contributor workflow document for splitting independent coverage work across parent/child worktrees.

The source of truth for individual work remains `issues/`. This document records the wave-level grouping, expected coverage movement, and focused gates.

## Epics

| Epic | Issue | Roadmap lane | Coverage gap |
|---|---|---|---|
| Builtin API Coverage Expansion | `I-20260512-BTAP7K` | W4 | `builtin-api`, `array-builtin` |
| Class Implementation Completion | `I-20260512-CA5S2K` | W5 | `class` |
| Async/Await Support | `I-20260512-ASYNC3` | W5 | `async` |
| Import/Export Module System | `I-20260512-MD7EX4` | W5 | `import-export` |
| TypeScript Erased Features + tsc/tsgo Ramp | `I-20260512-TSG6R2` | W2/W6 | `tsc`, `tsgo`, TypeScript erasure |
| Name Resolution Improvements | `I-20260512-NAM3R5` | W3 | `name-resolution` |

## Parent/Child Execution

Run the standard issue checks before spawning children:

```bash
mise run issue-lint
mise run issue-index
```

Spawn one child worktree per independent epic:

```bash
mise run spawn-worktrees -- \
  --base master \
  --prefix covexp \
  issues/I-20260512-BTAP7K.md \
  issues/I-20260512-CA5S2K.md \
  issues/I-20260512-ASYNC3.md \
  issues/I-20260512-MD7EX4.md \
  issues/I-20260512-TSG6R2.md \
  issues/I-20260512-NAM3R5.md
```

Each child reads this document and its assigned issue file before implementation. Parent merge review owns conflict resolution, coverage artifact regeneration, and final gate selection.

## Focused Gates

Builtin, class, async, module, and name-resolution work should use the smallest command set that proves the changed semantic surface:

```bash
cargo test -p ts2wasm-cli --test m2_node_diff
cargo test -p ts2wasm-cli --test m6_builtin_methods
mise run reference-coverage -- test262 --jsonl --sample 50 --jobs 4 --no-dashboard-data
```

TypeScript erased-feature and ramp work should include TypeScript reference coverage:

```bash
mise run reference-coverage -- tsc --limit 30
mise run reference-coverage -- tsgo --limit 20
```

When a command is unavailable because a reference corpus or external runtime is missing, record the unavailable tool and the nearest focused command in issue evidence.

## Merge Criteria

Before a child branch is merged:

- issue acceptance commands pass or have explicit blocker evidence;
- no unsupported label is reduced by hiding failures as skips;
- semantic pass, mismatch, runtime error, and build-only counts are reported separately;
- coverage dashboard artifacts are regenerated only by the parent integration pass;
- new docs point to `issues/` for future work instead of embedding TODO lists.
