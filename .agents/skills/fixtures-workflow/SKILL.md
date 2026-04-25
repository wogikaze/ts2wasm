---
name: fixtures-workflow
description: "Use when adding, editing, moving, or renaming fixtures under fixtures/ in this repository. Trigger phrases: add fixture, edit fixture, rename fixture, migrate fixtures, fixture path update, fixture reference sync."
---

# Fixtures workflow

Use this skill when a change touches `fixtures/**` or fixture path references caused by a `fixtures/**` change.

## Mise: run before you finish (required)

**You must** run the matching entries below and ensure they pass before calling the work done. Without `mise`, use `scripts/manager` with the same subcommand. First time: `mise trust` ([docs](https://mise.jdx.dev/cli/trust.html)).

- `mise run fmt` and `mise run nextest` (minimum)
- If compiler/runtime/Fixture references change: `mise run clippy` as well
- `mise run check-repo-smoke` as a light aggregate (fmt + script syntax + `issues` invariants)
- If reference coverage or fixtures in matrix were affected: `mise run update-coverage-matrix` / `mise run reference-coverage` (see `scripts/` for flags)

This skill is for fixture input files, fixture naming, fixture migration, and fixture reference synchronization. It may update script path references, but it must not redesign script behavior. Use `scripts-workflow` for script logic, script output schema, command-line options, CI script orchestration, or coverage pipeline behavior changes.

## Scope

In scope:

- `fixtures/**` TypeScript inputs for project-owned smoke, compile, iwasm, and Node differential tests
- fixture directory and file naming
- fixture moves, renames, and migrations
- fixture reference synchronization in:
  - `crates/cli/tests/**`
  - `scripts/**`
  - `docs/**`
  - `README.md`
  - `AGENTS.md`
  - `.github/workflows/**`
  - `artifacts/coverage/**` only when the document explicitly tracks project fixtures
- `TestRecord` suite/case strings and fixture-related metadata

Out of scope:

- changing shell script behavior
- changing script output formats such as JSONL, markdown reports, or generated coverage matrix rows
- changing script arguments or usage contracts
- changing CI job topology except path/reference synchronization caused by fixture moves
- changing reference corpus execution policy for `test262`, TypeScript compiler cases, or `typescript-go`

## Search Tool Policy

Prefer tools in this order:

1. `ast-grep` / `sg` for structural searches in Rust, TypeScript, JavaScript, JSON, YAML, and other supported languages.
2. `rg` (ripgrep) for broad text/path searches, literal fixture path searches, and shell/doc scans.

Do not use plain text grep when the question is structurally about code shape, such as `TestRecord` construction, fixture helper calls, or Rust test registration.

Use `rg` for:

- old fixture path references
- old directory names
- docs and shell scripts
- mixed old/new naming scans
- broad `fixtures/` inventory

Use `sg` for:

- `TestRecord { ... }` records
- Rust helper calls such as `assert_fixture_*`
- TypeScript/JavaScript fixture constructs
- YAML workflow path filters
- structured rewrite candidates

## Boundary With Scripts Workflow

Scripts are consumers of fixtures. A fixture workflow may update literal paths in scripts when a fixture path changed.

Allowed here:

- replace `fixtures/<old-path>` with `fixtures/<new-path>`
- update fixture directory globs after a migration
- update usage examples only when they mention a moved fixture
- update comments that list fixture directories
- run `scripts/check/shell-syntax.sh` after touching scripts

Not allowed here:

- add a new script option
- change JSONL/TestRecord/report field names
- change coverage ramp logic
- change regression gate thresholds
- change script parallelism behavior
- change script stdout/stderr contract
- change generated artifact policy

If a fixture rename reveals that a script needs different logic, stop the fixture-only change and hand off the script part to `scripts-workflow`.

## Naming Rules

1. Use semantic, domain-based directory names.
2. Use kebab-case for directories and files.
3. Avoid milestone-style opaque names such as `m1`, `m2`, `m6`.
4. Keep each fixture focused on a single observable behavior.
5. Prefer behavior names over implementation-layer names.
6. Do not mix unrelated semantic areas in one directory.
7. Do not create directories that duplicate docs milestone names.

Good examples:

```text
fixtures/primitives-control-flow/boolean-if.ts
fixtures/core-semantics/truthiness.ts
fixtures/arrays-objects/array-oob.ts
fixtures/builtins-and-io/stdin.ts
fixtures/classes-and-inheritance/class-super-method.ts
fixtures/modules-and-typed-optimizations/require-cache.ts
```

Bad examples:

```text
fixtures/m3/test1.ts
fixtures/new-tests/foo.ts
fixtures/misc/bar.ts
fixtures/runtime-fixes/case.ts
```

## Fixture Content Rules

Each fixture should make one behavior observable through stdout, compile success, compile failure, or runtime behavior.

Prefer:

- small source files
- stable stdout
- no wall-clock dependence
- no random output unless randomness is the tested API
- explicit observable result through `console.log`
- Node-compatible behavior unless the fixture is intentionally checking unsupported behavior
- one semantic feature per fixture

Avoid:

- large scenario fixtures that combine many features
- hidden dependency on execution order
- filesystem writes unless testing filesystem behavior
- environment variables unless testing `process.env`
- args unless testing `process.argv`
- encoding ambiguity in stdin/stdout fixtures

## Required Inventory Pass

Before changing fixture paths, build the impact list.

Run broad text/path searches with `rg`:

```bash
rg -n 'fixtures/<old-path>|<old-dir>/' crates scripts docs README.md AGENTS.md .github artifacts
rg -n '<old-file-name>|<old-dir>' crates scripts docs README.md AGENTS.md .github artifacts
rg -n 'fixtures/' crates/cli/tests scripts docs README.md AGENTS.md .github artifacts
```

Run structural Rust searches with `sg`:

```bash
sg run --lang rust -p 'TestRecord { $$$ }' crates/cli/tests crates/shared/src
sg run --lang rust -p 'assert_fixture_compiles($$$)' crates/cli/tests
sg run --lang rust -p 'assert_fixture_matches_node($$$)' crates/cli/tests
sg run --lang rust -p 'assert_fixture_matches_iwasm($$$)' crates/cli/tests
sg run --lang rust -p 'run_differential_test($$$)' crates/cli/tests
```

For YAML workflow path references, prefer structured search first:

```bash
sg run --lang yaml -p 'fixtures/**' .github/workflows
```

If the structural query is too narrow or invalid for the target file type, fall back to `rg`:

```bash
rg -n 'fixtures/' .github/workflows
```

For broad migrations, also run:

```bash
find fixtures -type f | sort
rg -n 'fixtures/' crates/cli/tests scripts docs README.md AGENTS.md .github artifacts
sg run --lang rust -p 'TestRecord { $$$ }' crates/cli/tests crates/shared/src
```

Use the result as the migration checklist. Do not rely on memory.

## Required Reference Update Pass

When fixture paths change, update all affected references in one change.

Check at minimum:

1. `crates/cli/tests/**`
   - direct fixture string arrays
   - helper functions that join `fixtures/`
   - `assert_fixture_compiles`
   - `assert_fixture_matches_node`
   - stdin/differential helpers
   - snapshot or temp wasm names derived from fixture paths

2. `TestRecord` metadata
   - `suite` should remain meaningful and path-aligned, usually `fixtures/<domain-dir>`
   - `case` should remain the fixture filename
   - `target` must stay correct
   - non-pass records must keep `reason` and `tracking`

3. `scripts/**`
   - literal fixture paths
   - fixture globs
   - examples in usage headers
   - benchmark sample fixtures
   - smoke fixture lists
   - script comments that describe fixture groups

4. `docs/**`
   - internal smoke fixture lists
   - testing policy examples
   - coverage or dashboard notes mentioning project fixtures
   - current implementation status
   - roadmap/gate text if it names the fixture group

5. Root/agent files
   - `README.md`
   - `AGENTS.md`
   - `.github/copilot-instructions.md`
   - `.agents/skills/**` only when the documented workflow becomes stale

6. CI/workflows
   - `.github/workflows/**` path triggers
   - workflow examples that call scripts over fixtures
   - required check reachability after path moves

7. Generated or semi-generated artifacts
   - update only when the artifact explicitly tracks project fixture counts or paths
   - do not hand-edit generated reference coverage tables unless the project docs say they are the committed source of truth

## Path and Suite Invariants

Keep path and suite strings consistent.

Preferred shape:

```text
fixture path: fixtures/<domain-dir>/<case>.ts
TestRecord.suite: fixtures/<domain-dir>
TestRecord.case: <case>.ts
```

Do not leave mixed old/new names:

```text
fixtures/m6/stdin.ts
fixtures/builtins-and-io/stdin.ts
suite: fixtures/m6
suite: fixtures/builtins-and-io
```

A directory migration is not complete until path strings, suite strings, docs, and script consumers agree.

## Add Fixture Workflow

1. Choose the domain directory.
2. Add one focused `.ts` file.
3. Add or update the smallest relevant Rust integration test.
4. If the fixture belongs to Node vs iwasm parity, wire it into the differential suite.
5. If the fixture is compile-only, wire it into the compile fixture suite.
6. If the fixture needs stdin/env/argv/fs behavior, document the input contract in the test helper or fixture name.
7. Update docs only when the fixture changes the documented coverage, supported subset, or gate evidence.
8. Run validation.

## Edit Fixture Workflow

1. Identify whether expected behavior changes or only the fixture source is clarified.
2. If expected stdout changes, update the corresponding test expectation.
3. If the fixture moves from unsupported to supported, update `TestRecord` classification and tracking.
4. If the fixture exposes a new semantic promise, update `docs/05-compatibility-and-semantics.md` or `docs/06-testing-and-coverage.md`.
5. Do not edit a fixture to make a failing test pass by weakening the tested behavior.
6. Run validation.

## Rename or Move Fixture Workflow

1. Use `git mv` for the file or directory.
2. Run the inventory pass.
3. Update all path strings and suite strings.
4. Update docs and CI path references.
5. Run script syntax checks if any script changed.
6. Run fixture-heavy tests.
7. Run the full standard gate unless the change is documentation-only and no fixture path changed.

## Validation

Always run:

```bash
cargo fmt --all --check
```

For fixture content changes, run the directly impacted tests first, then the project gate:

```bash
cargo nextest run -p ts2wasm-cli
```

For fixture-heavy or path migration changes, run:

```bash
cargo nextest run
```

If scripts were touched only for fixture path synchronization, also run:

```bash
scripts/check/shell-syntax.sh
```

If the moved fixture is used by differential execution, run the relevant differential/integration test. Examples:

```bash
cargo nextest run -p ts2wasm-cli --test m2_node_diff
cargo nextest run -p ts2wasm-cli --test m6_builtin_methods
cargo nextest run -p ts2wasm-cli --test m7_control_flow
cargo nextest run -p ts2wasm-cli --test m8_oop_classes
cargo nextest run -p ts2wasm-cli --test m9_modules
cargo nextest run -p ts2wasm-cli --test m10_node_apis
```

If the fixture requires `iwasm`, state whether `iwasm` was available. Do not report an iwasm-dependent check as passed if it was skipped because the tool was missing.

If docs mention coverage artifacts, validate the coverage matrix check when relevant:

```bash
scripts/gen/coverage-matrix.sh --check
```

Do not run reference corpus scripts for ordinary project fixture edits unless the change affects reference coverage, TestRecord schema, differential classification, or CI coverage scripts.

## Grep Gates

After any move or rename, run searches that should return zero old references.

Use `rg` for literal old path checks:

```bash
rg -n 'fixtures/<old-path>|<old-dir>/' crates scripts docs README.md AGENTS.md .github artifacts
rg -n 'suite: "fixtures/<old-dir>|suite = "fixtures/<old-dir>' crates
```

Use `sg` for structural verification in Rust tests:

```bash
sg run --lang rust -p 'TestRecord { $$$ }' crates/cli/tests crates/shared/src
sg run --lang rust -p 'assert_fixture_compiles($$$)' crates/cli/tests
sg run --lang rust -p 'assert_fixture_matches_node($$$)' crates/cli/tests
sg run --lang rust -p 'run_differential_test($$$)' crates/cli/tests
```

For broad migrations, run a consistency scan:

```bash
find fixtures -maxdepth 2 -type f | sort
rg -n 'fixtures/' crates/cli/tests scripts docs README.md AGENTS.md .github artifacts
sg run --lang rust -p 'TestRecord { $$$ }' crates/cli/tests crates/shared/src
```

## Common Traps

- Directory rename done but `TestRecord.suite` left old.
- Rust tests updated but scripts still compile old fixture paths.
- Scripts updated by changing behavior instead of only changing references.
- Docs list old fixture groups after migration.
- CI path filters do not include the new files that should trigger checks.
- `artifacts/coverage/reference-coverage-matrix.md` is hand-edited for project fixtures even though it tracks reference corpus data.
- Mixed old/new naming remains after partial migration.
- Fixture is added but not wired into any test.
- Fixture passes by compile-time evaluation instead of runtime behavior when runtime behavior is the intended gate.
- Unsupported fixture lacks `reason` or `tracking`.
- Stdin/env/fs fixture has an implicit host contract that tests do not reproduce.
- Text search is used where `sg` would catch structural call sites more reliably.

## Output Checklist

Every fixture workflow result must report:

1. Added, edited, moved, or removed fixture paths.
2. Updated reference files.
3. TestRecord suite/case changes, if any.
4. Script files touched only for path synchronization, if any.
5. Docs or CI files updated, if any.
6. Validation commands and results.
7. `rg` gate results for old paths after rename/migration.
8. `sg` structural gate results for Rust fixture references.
9. Intentional non-updated areas and reason.
10. Tooling limitations, such as missing `rg`, `sg`, or `iwasm`, if applicable.

## Handoff Packet

Use this format when handing the result to gatekeeper or another agent.

```text
Fixture workflow handoff

Scope:
- Added:
- Edited:
- Moved:
- Removed:
- Not changed:

Reference sync:
- crates/cli/tests:
- scripts:
- docs:
- README/AGENTS:
- .github/workflows:
- artifacts:

TestRecord:
- suite changes:
- case changes:
- non-pass reason/tracking changes:

Validation:
- cargo fmt --all --check:
- cargo nextest run -p ts2wasm-cli <impacted>:
- cargo nextest run:
- scripts/check/shell-syntax.sh:
- scripts/gen/coverage-matrix.sh --check:
- iwasm-dependent checks:

Search gates:
- rg old fixture path references:
- rg old suite references:
- sg TestRecord structural scan:
- sg fixture helper call scan:

Risks:
- Runtime behavior changed:
- Script behavior changed:
- CI trigger changed:
- Coverage artifact changed:

Intentional non-updates:
- <file or area>: <reason>
```
