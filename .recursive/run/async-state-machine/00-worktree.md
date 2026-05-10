# Phase 0: Worktree Isolation

## Worktree Location

- **Path**: `.worktrees/async-state-machine/`
- **Branch**: `recursive/async-state-machine`
- **Git ignore**: `.worktrees/` is NOT in `.gitignore` (standard git worktree convention)

## Baseline

### Setup commands

```bash
# Worktree creation
git worktree add .worktrees/async-state-machine -b recursive/async-state-machine

# Setup
# No additional setup needed (cargo build used via nextest)
```

### Baseline test results

**m12_async_await tests** (pre-implementation):

```
PASS build_smoke_async_return
PASS build_smoke_await_sequence
PASS build_smoke_async_exception
SKIP semantic_diff_async_return  (ignored: async/await semantic implementation in progress)
SKIP semantic_diff_await_sequence (ignored: async/await semantic implementation in progress)
SKIP semantic_diff_async_exception (ignored: async/await semantic implementation in progress)
```

All 3 build_smoke pass, 3 semantic_diff explicitly ignored.

**Fast gate**:

```
cargo fmt --all --check  => pass
check-fast-gate --skip-nextest => OK
```

## Diff Basis

- **Baseline type**: git commit
- **Baseline reference**: `f678b712d` (HEAD at worktree creation)
- **Comparison reference**: working tree changes
- **Normalized baseline**: `git diff f678b712d`
- **Normalized comparison**: `git diff --cached`
- **Normalized diff command**: `git diff f678b712d HEAD`

## Router State

Worktree is fresh. Router files not yet configured (no delegated routing needed for this run).

## TODO

- [x] Phase 0: Worktree isolation complete
- [ ] Phase 1: AS-IS analysis
- [ ] Phase 2: TO-BE plan
- [ ] Phase 3: Implementation (TDD)
- [ ] Phase 4: Validation
- [ ] Phase 6: DECISIONS.md update
- [ ] Phase 7: STATE.md update
- [ ] Phase 8: Memory impact
