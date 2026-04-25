---
name: scripts-workflow
description: Use when adding/editing scripts under scripts/. Covers layout conventions, shell rules, output contracts, validation.
---

# Scripts workflow

**Discovery:** the repo entry is `scripts/manager` and root `mise.toml` (list: `mise tasks`); avoid making people read every `scripts/*.sh` to find usage. When you add a script, register it in `manager` and a `[tasks.*]` in `mise.toml`. **Layout (first tier):** `scripts/check/` (static, non-destructive), `scripts/gate/` (pass/fail), `scripts/gen/` (refresh tracked generated artifacts), `scripts/run/` (execute/measure), `scripts/report/` (human-facing formatting), `scripts/perf/` (benchmarks), `scripts/dev/` (local setup), `scripts/lib/` (sourced helpers only; not executed). Deprecated top-level names may remain as thin `exec` wrappers during migration. **Harness baseline:** `scripts/check_harness_installation.sh` inventories toolchain + P0 `check_*.sh` and runs the rest of the project gates; optional strict Rust warnings: `TS2WASM_NEXTEST_DENY_WARNINGS=1` (see `issues/open/011-*.md` until the tree is clean).

## Table of Contents

- [Mise: run before you merge a script change](#mise-run-before-you-merge-a-script-change-required)
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

## Mise: run before you merge a script change (required)

**Execute all that apply; do not ship a script change without a green local gate.** First time: `mise trust` ([docs](https://mise.jdx.dev/cli/trust.html)). Without `mise`, use `scripts/manager` with the same subcommand.

- Always: `mise run check-scripts` (plus `mise run fmt` if the script is invoked from tests or the diff touches Rust)
- `mise run check-repo-smoke` after touching `issues` paths or the manager
- For coverage/ CI scripts: also run the same command family you would run in `scripts` docs (e.g. `mise run reference-coverage` with a small limit when that script supports it)
- `mise tasks` to confirm your new `mise run <task>` appears after you add it to `mise.toml`

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

Always run:

- cargo fmt --all --check
- scripts/check/shell-syntax.sh（`bash -n` のみ。syntax OK は runtime OK を意味しない）
- bash -n <touched-script>

Run the touched script with a representative command（`shell-syntax.sh` はこれに代わらない）。

Examples:

- mise run reference-coverage -- test262 --limit 1
- mise run reference-coverage -- tsc --limit 1
- mise run reference-coverage -- tsgo --limit 1
- mise run check-coverage-matrix
- mise run check-fast-gate --skip-nextest
- mise run check-manifest-imports
- mise run check-test-records-schema <file.jsonl>
- mise run check-fixture-catalog
- mise run check-architecture-rules
- mise run check-compiler-diagnostics
- mise run check-coverage <base-doc> <current-doc>
- mise run test262 -- --sample 1 --jobs 1
- mise run check-regression <results.jsonl> --baseline <baseline.json>
- mise run report-differential --markdown <tmp.md> --html <tmp.html>

Run tests:

- cargo nextest run for impacted tests at minimum
- cargo nextest run when output contracts, fixture references, coverage classification, or gate behavior changes

When the touched script consumes fixtures, also run at least one fixture-heavy or differential path that exercises the changed reference logic.

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
