---
id: 5419
title: "W6: Fix test262-metadata rejection — allow compilation despite unknown features"
type: feature
area: scripts
class: implementation-ready
priority: P1
depends_on: []
blocks: []
created: 2026-05-08
updated: 2026-05-08
---

## Summary

Fix the test262 coverage runner to not reject files with unknown `features` in YAML frontmatter. Currently 27,417 files (51% of corpus) are rejected at metadata level without even attempting compilation. Instead, log unknown features and proceed to compile.

## Problem

At full corpus (53,449 files), 27,417 are marked `UnsupportedTest262Metadata/test262-metadata` because the runner checks `features` against `SUPPORTED_FEATURES` and rejects any file with an unrecognized feature. This masks the real compilation blockers.

Problem: 27,417 test262 files rejected at metadata level without compilation attempt.

## Root cause

`scripts/lib/test262_harness.py:184-191`:
```python
@property
def unsupported_reason(self):
    for feature in self.features:
        if feature not in SUPPORTED_FEATURES:
            return f"test262 feature `{feature}` is not supported by this runner slice"
```

This prevents the compiler from even seeing the file. The fix: log the unknown feature but return `None` (proceed to compilation).

## Desired final state

- test262 files with unknown features proceed to actual compilation
- Unknown features are logged/stats-collected for triage
- `test262-metadata` unsupported count drops from 27,417 to near 0
- The actual compilation results (build_pass, UnsupportedSyntax, etc.) are visible

## Scope

In scope:

- [x] Change `unsupported_reason` property to log unknown features but not reject
- [x] Add a warning/stats counter for unknown features
- [x] Run `mise run reference-coverage -- test262 --limit 2000` to verify the change
- [x] Run `mise run update-coverage-matrix` to refresh data

Out of scope:

- Adding all test262 features to SUPPORTED_FEATURES (too many, would be stale)
- Changing the actual compiler to support new features (separate issues)
- Fixing `has_scope` / raw mode logic in the runner

## Affected paths

Expected:

- `scripts/lib/test262_harness.py` — change `unsupported_reason` to warn instead of reject
- `scripts/run/reference-coverage.py` — add unknown feature logging/statistics

Do not touch:

- `crates/` — Rust code out of scope
- `docs/` — docs out of scope (unless evidence format changes)
- `fixtures/` — fixtures out of scope

## Acceptance criteria

- [x] `mise run reference-coverage -- test262 --limit 2000` shows `test262-metadata` unsupported count decreased from 165 to near 0
- [x] Unknown features are logged to stderr or a stats file for triage
- [x] `mise run update-coverage-matrix -- --check` passes

## Validation

Required commands:

```sh
mise run reference-coverage -- test262 --limit 2000
mise run update-coverage-matrix
mise run update-coverage-matrix -- --check
```

## Notes

- The `unsupported_reason` property is at `scripts/lib/test262_harness.py:184`
- Change from `return f"test262 feature..."` to `print(f"warn: unknown feature...", file=sys.stderr)` and `return None`
- Also reconsider `UNSUPPORTED_FLAGS = ("IsHTMLDDA",)` — IsHTMLDDA is already in SUPPORTED_FEATURES, so the flag check may be redundant
- After the fix, many files that were `test262-metadata` will become `UnsupportedSyntax`, `UnresolvedName`, or `build_pass` — this is expected and desirable

## False-done audit

**truly-done** (5419)

- Implementation commits: verified via `git log --oneline --all --grep=5419`
- Completion evidence: filled with specific commit hashes and validation results
- Acceptance criteria: all checked as met

This issue has repo-local close evidence with implementation commits and validation commands.
