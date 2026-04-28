# Child Assignment: 052 JSON number/space continuation

Child run id: `052-json-number-space-20260428T094954Z`
Worktree: `/home/wogikaze/wgkz/ts2wasm-052-json-number-space-20260428T094954Z`
Branch: `agent/052-json-number-space-20260428T094954Z`
Parent branch at assignment: `master` @ `0b4cfb1`

You are not alone in this repository. Other agents may be editing other worktrees; do not revert, overwrite, or depend on changes outside this branch. Do not merge to `master`.

## Assigned Issue List

1. `issues/open/052-implement-json.md`

## Objective

Complete one narrow, reference-backed JSON continuation slice without broadening the issue beyond what can be validated in this cycle.

Preferred slice order:

1. Try a narrow `JSON.stringify` third-argument `space` support for object-literal/string-literal or simple object subset, with Node/iwasm differential fixture evidence.
2. If the `space` path is unsafe, implement a narrow numeric JSON continuation such as decimal/exponent parsing or stringifying an already-supported numeric form, again with direct Node/iwasm evidence.
3. If implementation is unsafe, add a precise issue-052 unsupported diagnostic plus regression coverage for the attempted form and record PROGRESS/BLOCKED with evidence.

Do not claim issue 052 done unless the full JSON acceptance criteria are satisfied. Keep unsupported diagnostics for function replacers and unsupported array replacer contents/forms.

## Allowed Files

- `crates/backend-wasm/src/`
- `crates/compiler/src/` only for JSON lowering/diagnostics
- `crates/ir/src/` only if a JSON call representation change is strictly required
- `crates/cli/tests/`
- `fixtures/builtins-and-io/`
- `issues/open/052-implement-json.md`
- `current-state.md` only if implementation facts changed
- `reports/agents/052-json-number-space-20260428T094954Z/`
- `reports/runs/052-json-number-space-20260428T094954Z/`

## Forbidden Files

- `docs/`
- Module-system fixtures or issue 233 files
- Any issue other than 052
- Parent branch or any other agent worktree

## Required Validation

Run the narrow JSON gates and direct evidence for any new fixture:

```sh
cargo fmt --all --check
cargo nextest run -E 'test(json)'
cargo nextest run -p ts2wasm-cli json
node <new-json-fixture>
cargo run -q -p ts2wasm-cli -- build <new-json-fixture> -o /tmp/ts2wasm-052-json-continuation.wasm
iwasm /tmp/ts2wasm-052-json-continuation.wasm
scripts/manager check-issue-health
scripts/manager check-agent-state
```

Run full `cargo nextest run` if runtime/helper code changes are broad or if you attempt to close issue 052.

## Reporting

- Write `reports/runs/052-json-number-space-20260428T094954Z/cycle_report.md`.
- Write a machine-readable `test_report.json` when practical and validate it with the repo schema.
- Attempt `scripts/manager discord-report --run-id 052-json-number-space-20260428T094954Z`; if the webhook is unavailable, save the deferred payload/error and continue.
- Commit all validated useful work.
- End with exactly one parent event line, for example:

```text
PARENT_EVENT: PROGRESS issue=052 branch=agent/052-json-number-space-20260428T094954Z commit=<hash> validation="<summary>" report=reports/runs/052-json-number-space-20260428T094954Z/cycle_report.md merge_request=yes
```

Use `DONE` only if the issue file is moved to `issues/done/`, index regenerated, full close requirements met, and all acceptance criteria are verified.
