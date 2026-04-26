---
name: scripts-workflow
description: Use when adding/editing scripts under scripts/. Covers layout conventions, shell rules, output contracts, validation.
---

# Scripts workflow

**Discovery:** the repo entry is `scripts/manager` and root `mise.toml` (list: `mise tasks`); avoid making people read every `scripts/*.sh` to find usage. When you add a script, register it in `manager` and a `[tasks.*]` in `mise.toml`. **Layout (first tier):** `scripts/check/` (static, non-destructive), `scripts/gate/` (pass/fail), `scripts/gen/` (refresh tracked generated artifacts), `scripts/run/` (execute/measure), `scripts/report/` (human-facing formatting), `scripts/perf/` (benchmarks), `scripts/dev/` (local setup), `scripts/lib/` (sourced helpers only; not executed). Deprecated top-level names may remain as thin `exec` wrappers during migration. **Harness baseline:** `scripts/manager check-harness-installation` inventories toolchain + P0 checks and runs the rest of the project gates; optional strict Rust warnings: `TS2WASM_NEXTEST_DENY_WARNINGS=1` (see `issues/open/011-*.md` until the tree is clean).

## Table of Contents

- [Manager: auto-execute after making changes](#manager-auto-execute-after-making-changes-required)
- [Manager / Entry Point Rules](#manager--entry-point-rules)
- [Migration / Old Reference Rules](#migration--old-reference-rules)
- [Issue / Index Script Rules](#issue--index-script-rules)
- [Agent State and Run Report Rules](#agent-state-and-run-report-rules)
- [Repo Root and Script Location Rules](#repo-root-and-script-location-rules)
- [Scope](#scope)
- [Core Rules](#core-rules)
- [Fixture Boundary Rules](#fixture-boundary-rules)
- [Input Selection Rules](#input-selection-rules)
- [Output Contract Rules](#output-contract-rules)
- [Temporary File and Artifact Rules](#temporary-file-and-artifact-rules)
- [Hermeticity and Reproducibility Rules](#hermeticity-and-reproducibility-rules)
- [Regression Gate Rules](#regression-gate-rules)
- [Script Change Classification](#script-change-classification)
- [Validation](#validation)
- [Common Traps](#common-traps)
- [Output Checklist](#output-checklist)
- [Related Skills](#related-skills)

## Manager: auto-execute after making changes (required)

**Automatically execute the following after making script changes.** Use `scripts/manager` as the primary repo entry. `mise run <task>` is optional sugar for the same tasks.

- Always: `scripts/manager check-scripts` (plus `scripts/manager fmt` if the script is invoked from tests or the diff touches Rust)
- `scripts/manager check-repo-smoke` after touching `issues` paths or the manager
- For coverage/ CI scripts: also run the same command family you would run in `scripts` docs (e.g. `scripts/manager reference-coverage` with a small limit when that script supports it)
- `mise tasks` to confirm your new `mise run <task>` appears after you add it to `mise.toml`
- **Auto-commit changes after verification passes** (commit message based on change description)

## Manager / Entry Point Rules

`scripts/manager` is the canonical executable entrypoint.
`scripts/manager.py` may contain the implementation, but callers must use `scripts/manager`.

Required rules:

1. When adding a command, register it in all applicable places:
   - `scripts/manager.py`
   - `mise.toml`
   - docs / skills that mention the command
   - CI workflow path filters if the command affects CI behavior
2. Keep `scripts/manager` as a thin executable shim:
   - it must exist
   - it must be executable
   - it must dispatch to `scripts/manager.py`
3. Do not document direct calls to implementation files unless the file is intentionally public.
   - Prefer: `scripts/manager check-issue-health`
   - Avoid: `python scripts/check/issue-health.py`
4. After manager or script command changes, run:
   - `scripts/manager check-scripts`
   - `scripts/manager check-repo-smoke`
   - `scripts/manager check-agent-state`
5. After adding a `mise.toml` task, run:
   - `mise tasks`

## Migration / Old Reference Rules

Script migrations must remove stale command references in the same change.

Before finishing any script rename, `.sh` to `.py` migration, or manager command rename, run:

```sh
rg 'scripts/check_.*\.sh|update_issue_index|issue-queue\.py|update-issue-index\.sh|fixture-differential\.sh|check_fast_gate\.sh|check_manifest_imports\.sh' .
```

If any hit remains, classify it explicitly:

- valid compatibility wrapper
- historical note in completed issue
- stale reference to fix now

Do not leave stale references in:

- `.agents/skills/**`
- `.agents/prompts/**`
- `.github/workflows/**`
- `.githooks/**`
- `README.md`
- `AGENTS.md`
- `issues/open/**`
- `issues/index.md`

## Issue / Index Script Rules

Issue queue scripts are infrastructure-critical. Do not let checker and generator drift.

Required rules:

1. Shared parsing/rendering must live in `scripts/lib/`.
2. `scripts/check/issue-health.py` and `scripts/gen/update-issue-index.py` must use the same parser and table renderer.
3. `scripts/manager update-issue-index --check` must fail if generated table content differs, not only if IDs are missing.
4. `scripts/manager check-issue-health` must verify:
   - duplicate IDs
   - open/done conflicts
   - missing dependencies
   - stale generated index
   - missing repo-owned backticked paths
5. `reference/**` paths are external corpus references, not normal repo-owned paths. Do not fail issue health solely because `reference/**` is not cloned.
6. YAML issue frontmatter support is limited to the documented single-line format unless a real YAML parser is introduced.

Allowed issue frontmatter shape:

```yaml
---
id: 026
title: "Migrate backend module to backend-wasm crate"
type: refactor
area: backend
class: implementation-ready
priority: P1
depends_on: [024, 025]
---
```

Unsupported unless explicitly implemented:

```yaml
depends_on:
  - 024
  - 025
```

## Agent State and Run Report Rules

Autonomous-loop scripts must preserve auditable state.

Required preflight:

```sh
scripts/manager check-agent-state
scripts/manager check-repo-smoke
```

Rules:

1. `check-agent-state` must fail when required schema validation dependencies are missing.
2. `jsonschema` must be available through the documented dev environment.
3. State schemas and examples must stay consistent.
4. Run/cycle scripts must write under `reports/runs/<run_id>/`.
5. Report generators must not overwrite unrelated runs.
6. Human-readable cycle notes and machine-readable test reports are separate artifacts.

Minimum run directory shape:

```text
reports/runs/<run_id>/
  cycle_report.md
  test_report.json        # when command execution is captured
  commands/
    001.stdout
    001.stderr
```

## Repo Root and Script Location Rules

Repo-root mistakes are high-risk.

Required rules:

1. Every script must resolve repo root from its own file location or use manager-provided repo root.
2. Scripts under `scripts/check/`, `scripts/gate/`, `scripts/gen/`, `scripts/run/`, `scripts/report/`, `scripts/perf/`, `scripts/dev/` must not assume they are one level below repo root.
3. For shell scripts under `scripts/<tier>/foo.sh`, use:

```bash
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"
```

1. For shell scripts under `scripts/foo.sh`, use:

```bash
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"
```

1. For Python scripts, use `Path(__file__).resolve().parents[N]` and verify the expected root contains `README.md` or `.git`.
2. `scripts/lib/` files are helpers. They are imported or sourced, not executed directly.

Use this skill only for scripts/ changes.

This skill owns shell script behavior, reliability, CLI contract, and machine-readable output stability.
It does not own fixture content, fixture naming, fixture migration, or fixture path taxonomy.

## Scope

In scope:

- scripts/**/*.sh and `scripts/manager` maintenance
- script usage header updates
- script option parsing
- script reliability and reproducibility improvements
- coverage / benchmark / regression gate script behavior
- reporter script output stability
- reference-suite runner behavior
- CI-facing output contracts

Out of scope:

- editing fixture files under fixtures/**
- renaming fixture directories or files
- changing fixture taxonomy
- changing TestRecord status semantics
- changing docs policy without updating the script contract that consumes it

Use `fixtures-workflow` together with this skill when a script change requires fixture path migration or fixture reference synchronization.

## Core Rules

1. Keep `set -e` or stricter existing options intact.
2. Prefer `#!/usr/bin/env bash` plus `set -euo pipefail` for scripts that use bash arrays, `mapfile`, `[[ ]]`, associative arrays, or process substitution.
3. Prefer POSIX-safe shell only when the script is already POSIX-compatible.
4. Quote paths and variables.
5. Resolve `repo_root` from the script location, then `cd "$repo_root"` before project-relative access.
6. Do not rely on the caller's current working directory.
7. Do not hardcode absolute project paths.
8. Validate required tools with `command -v` before first use when missing tools would produce confusing failures.
9. Keep stdout stable when it is consumed by CI, JSONL parsers, markdown table parsers, or other scripts.
10. Send human progress logs to stderr when stdout is machine-readable.
11. If behavior, options, defaults, or output format change, update the usage header in the same file.
12. If a docs page documents the script command or output contract, update that docs page in the same change.
13. If a contract (e.g., manifest schema, JSON format, TestRecord schema) changes, update consuming scripts in the same commit to avoid schema/script mismatch.

## Fixture Boundary Rules

Scripts may consume fixtures.
Scripts must not become the source of truth for fixture taxonomy.

Allowed:

- iterating over `fixtures/**` for smoke or differential runs
- selecting a small representative fixture set for validation
- compiling/running fixture files by stable semantic path
- producing TestRecord entries for fixture cases

Restricted:

- adding new fixture paths inside scripts
- renaming fixture paths inside scripts
- changing fixture directory categories
- silently replacing old fixture paths with new ones
- encoding milestone-style fixture group names such as `m1`, `m2`, `stream-g`

When fixture paths are touched or fixture references are changed, run the fixture reference update pass from `fixtures-workflow` in the same change:

- crates/cli/tests/**
- scripts/**
- docs/**
- TestRecord suite strings and related metadata

Recommended searches before finalizing a script that references fixtures:

- `fixtures/`
- `fixtures/<changed-dir>`
- `"<changed-dir>/"`
- `TestRecord`
- `suite`
- `case`

## Input Selection Rules

1. Sort discovered files before iteration.
2. Use stable locale-sensitive behavior explicitly where needed, for example `LC_ALL=C sort`.
3. Validate numeric options such as `--limit`, `--sample`, and `--jobs`.
4. Make sampling deterministic unless the usage header explicitly documents randomness.
5. For parallel runners, preserve deterministic output ordering or document that output ordering is intentionally nondeterministic.
6. Never let skipped or missing directories silently look like success unless the usage header documents that behavior.
7. For reference suites, distinguish:
   - repository-owned fixtures under `fixtures/**`
   - external reference corpora under `reference/**`
   - generated artifacts under `artifacts/**`

## Output Contract Rules

Machine-readable output is a compatibility contract.

For JSONL TestRecord output:

- one JSON object per line
- no progress logs on stdout
- include at least `suite`, `case`, `target`, `status`
- include `reason` and `tracking` for `unsupported`, `blocked`, and `skip-with-reason`
- do not invent status strings outside the canonical schema
- do not collapse `unsupported`, `blocked`, and `fail` into one bucket
- preserve field names unless all consumers are updated in the same change

For markdown coverage tables:

- preserve marker comments such as `<!-- coverage-table:start -->`
- preserve column order unless all readers are updated
- preserve numeric columns as parseable integers or decimals
- avoid presentation-only formatting in machine-parsed cells

For human reports:

- keep generated file paths explicit
- include enough reproduction command text to rerun the same sample
- do not mix policy text with generated measured results

## Temporary File and Artifact Rules

1. Use `mktemp -d` for temporary work directories.
2. Always install a cleanup trap.
3. Quote cleanup paths in traps.
4. Write generated persistent results under `artifacts/` or a user-provided output path, not under `fixtures/`.
5. Do not mutate fixture files during script execution.
6. Do not rely on `/tmp` layout except through `mktemp` or `${TMPDIR:-/tmp}`.
7. Make check modes non-mutating, or restore files before exit.
8. For benchmark or coverage scripts, record enough metadata to reproduce the run:
   - command
   - suite
   - limit/sample
   - target
   - runner
   - timestamp
   - git commit when available

## Hermeticity and Reproducibility Rules

1. Prefer repository files, declared reference directories, and generated build outputs over network access.
2. Do not fetch external resources from scripts unless the script is explicitly an installer/fetcher and documents it.
3. Do not depend on user-specific global state without a clear preflight error.
4. Do not require untracked local files unless the usage header names them.
5. Avoid ambient environment variables. When used, document them in the usage header.
6. Prefer explicit CLI flags over hidden environment behavior.
7. Keep test and coverage scripts re-runnable from a clean checkout after documented setup.

## Regression Gate Rules

Regression gates must fail on real regressions and avoid hiding coverage debt.

Required behavior for gates:

- fail if executed count decreases
- fail if fail count increases
- warn or fail explicitly when unsupported/blocked increases, according to the documented policy
- never count `unsupported`, `blocked`, or `skip-with-reason` as pass
- print the compared baseline and current files when failing
- exit nonzero on gate failure
- keep error messages stable enough for CI logs and reviewers

## Script Change Classification

Before editing, classify the script change:

1. syntax-only cleanup
2. option parsing change
3. output format change
4. fixture consumption change
5. reference-suite selection change
6. coverage/gate policy change
7. benchmark measurement change
8. generated artifact update

Use the strictest relevant validation group below.

## Validation

Always run the smallest valid set, but never stop at syntax-only checks.

For any script change:

```sh
scripts/manager check-scripts
bash -n <touched-shell-script>   # only when a shell script changed
scripts/manager check-repo-smoke
```

For manager, issue, state, or generated-index scripts:

```sh
scripts/manager update-issue-index --check
scripts/manager check-issue-health
scripts/manager check-agent-state
scripts/manager check-repo-smoke
```

For CI workflow changes:

```sh
scripts/manager check-repo-smoke
scripts/manager check-fast-gate --skip-nextest
```

For coverage/reference/test262 scripts:

```sh
scripts/manager update-coverage-matrix --check
scripts/manager check-coverage-gate <base-doc> <current-doc>
scripts/manager test262 --sample 1 --jobs 1
```

For scripts that produce JSONL/TestRecord:

```sh
scripts/manager check-test-records-schema <file.jsonl>
```

For scripts that consume fixtures:

```sh
scripts/manager check-fixture-catalog
scripts/manager check-fast-gate --skip-nextest
```

For Rust-impacting script changes:

```sh
scripts/manager fmt
cargo nextest run
```

## Common Traps

- Script update silently changes stdout format
- Human logs are printed into JSONL stdout
- Usage header no longer matches actual options
- `#!/bin/bash` is used when repo convention expects `#!/usr/bin/env bash`
- POSIX-safe claim is made while using bash-only syntax
- `local` is used outside a function
- Arrays or associative arrays are added without bash shebang
- `grep -P` is introduced without portability consideration
- `find` output is not sorted
- Unquoted paths break on spaces
- Temporary files are written into the repository root
- Check mode leaves generated files modified
- Fixture paths are changed only in scripts, not tests/docs/TestRecord metadata
- Fixture directory rename leaves old suite strings
- Coverage count improves because unsupported/blocked grew
- Reference corpus missing locally is treated as all-pass
- Parallel jobs produce nondeterministic machine-readable output
- Benchmark script changes measurement conditions without recording metadata
- `scripts/manager.py` exists but `scripts/manager` shim is missing
- docs/CI/hooks call `scripts/manager` but only direct Python entrypoints were tested
- `.sh` script is migrated to `.py` but workflow path filters still watch the old `.sh`
- checker and generator parse the same file with different logic
- issue index check only checks ID presence, not table content drift
- `reference/**` is treated as required repo-owned content
- `check-agent-state` silently passes without schema validation dependency
- `repo_root` is computed as `scripts/` instead of repository root
- `source scripts/lib/common.sh` is relative to the wrong tier directory
- `replace_generated_block()` drops final newline and causes endless stale-index diffs
- generated block marker absence is ignored
- syntax check passes but representative runtime command was never executed
- run report directory is created but no machine-readable command result is captured

## Related Skills

- fixtures-workflow: for fixture path updates when scripts reference fixtures
- docs-workflow: for documentation updates when script contracts change
- issues-workflow: for tracking script behavior changes

## Output Checklist

1. Which script files changed
2. Script change classification
3. Behavior delta before/after
4. Output contract delta, or `none`
5. Fixture/reference path delta, or `none`
6. Docs/usage header updates
7. Commands run for validation
8. Generated artifacts changed, or `none`
9. Remaining risks
