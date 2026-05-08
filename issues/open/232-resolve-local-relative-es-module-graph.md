---
id: 232
title: "Resolve local relative ES module graph"
type: feature
area: compiler/frontend
class: implementation-ready
priority: P1
depends_on: [231]
blocks: [233, 234]
created: 2026-04-28
updated: 2026-04-28
completed: 2026-04-28
status: done
---

## Summary

Build a deterministic module graph for static local relative ES module specifiers after the parser can represent module declarations.

## Problem

The compiler currently builds a single entry file and does not resolve static ES module specifiers into source files, module IDs, or diagnostics. Import/export lowering needs a checked module graph before it can bind names safely.

## Desired final state

Given an entry file, static local relative imports and re-exports resolve to canonical source files with stable module IDs, cycle detection, and source diagnostics for missing or unsupported specifiers.

## Scope

In scope:

- [x] Resolve `./` and `../` module specifiers relative to the importing file
- [x] Support `.ts` and `.js` source files with deterministic extension handling documented in tests
- [x] Build a module graph from the entrypoint without executing modules
- [x] Detect missing files and non-local/bare specifiers with issue-linked diagnostics
- [x] Preserve module IDs and source paths for lowering

Out of scope:

- Package resolution, `node_modules`, import maps, and TypeScript path mapping
- Dynamic import and CommonJS `require()`
- Export binding lowering or backend execution

## Affected paths

Expected:

- `crates/compiler/src/`
- `crates/frontend/src/`
- `crates/ir/src/`
- `crates/cli/tests/`
- `fixtures/module-system/`

Do not touch:

- `crates/backend-wasm/`
- `crates/runtime-abi/`
- `docs/`

## Acceptance criteria

- [x] Entry builds collect all reachable local relative ES modules exactly once
- [x] Module graph ordering is deterministic and covered by tests
- [x] Missing relative modules produce source diagnostics with the importing file/span
- [x] Bare specifiers are rejected with an unsupported diagnostic, not silently treated as paths
- [x] Cycles are detected and either represented safely or diagnosed explicitly

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-compiler
cargo nextest run -p ts2wasm-cli module
```

Impacted commands:

```sh
cargo run -q -p ts2wasm-cli -- dump fixtures/module-system/static-entry.ts --ast --resolved
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] not affected

Current state:

- [x] updated: `current-state.md` (repo root)

Follow-up issues:

- [x] downstream binding/lowering remains tracked by issues 233 and 234

## Notes

Keep path resolution local and deterministic. Do not implement Node package resolution in this issue.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `d96750d` issue-232: close module graph contract

Validation result:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-compiler module_graph: PASS (4 tests)
cargo nextest run -p ts2wasm-compiler: PASS (35 tests)
cargo nextest run -p ts2wasm-cli module: PASS (12 tests, 218 skipped)
cargo nextest run: PASS (363 tests, 4 skipped)
mise run update-issue-index: PASS
mise run update-issue-index -- --check: PASS
mise run check-issue-index: PASS
mise run check-issue-health: PASS
mise run check-agent-state: PASS
mise run discord-report -- --run-id 232-module-graph-close-audit-20260428T091725Z: DEFERRED (DISCORD_WEBHOOK_URL missing; payload/error saved)
date: 2026-04-28
```

Remaining risks:

- none

## Close evidence

2026-04-28 child worker `232-module-graph-close-audit-20260428T091725Z` audited the accumulated issue-232 graph diagnostics and cycle coverage, then closed the remaining contract gap:

- Exposed `build_entry_module_graph` from the compiler crate.
- Exposed read-only `ModuleGraph`, `ModuleNode`, and `ModuleDependency` contracts with stable module IDs, canonical source paths, dependency specifiers, dependency target IDs, and resolved dependency paths.
- Kept graph construction validation-only in the build and dump pipelines; import/export binding and execution semantics remain out of scope for issues 233 and 234.

Acceptance evidence:

- Reachable local relative modules exactly once: `module_graph::tests::builds_deterministic_entry_graph_and_deduplicates_modules`.
- Deterministic ordering and `.ts` before `.js`: `module_graph::tests::builds_deterministic_entry_graph_and_deduplicates_modules`.
- Missing relative diagnostics with source span and importing path: `module_graph::tests::rejects_missing_relative_module_at_specifier_span`.
- Bare specifier unsupported diagnostic: `module_graph::tests::rejects_bare_module_specifier_at_specifier_span`.
- Cycle behavior safely represented with existing module IDs: `module_graph::tests::represents_static_local_cycles_with_existing_module_ids`.
- Module IDs and paths exposed for downstream work: public compiler exports `build_entry_module_graph`, `ModuleGraph::modules`, `ModuleGraph::entry`, `ModuleGraph::module`, `ModuleNode::id`, `ModuleNode::path`, `ModuleNode::dependencies`, `ModuleDependency::specifier`, `ModuleDependency::resolved_module_id`, and `ModuleDependency::resolved_path`.

## Progress evidence

2026-04-28 child worker `232-module-graph-diagnostics-20260428T085234Z` completed an initial compiler/frontend module graph diagnostic slice:

- Added compiler-side static module specifier collection for parsed entry files and reachable local relative modules.
- Resolved local `./` and `../` specifiers deterministically as explicit `.ts` / `.js` files, or extensionless `.ts` then `.js`.
- Rejected bare/non-local specifiers with issue-232 unsupported diagnostics at the specifier span.
- Rejected missing local relative files with issue-232 diagnostics at the importing specifier span, including the importing path and candidate files tried.
- Added compiler tests for deterministic entry graph ordering, duplicate reachable module collection exactly once, `.ts` preference over `.js`, bare specifier rejection, and missing relative module rejection.
- Updated module CLI diagnostic tests and fixtures so source-bearing static declarations now prove issue-232 graph diagnostics, while local/export-only declarations still stop before lowering/emission under issue-055.
- Added `fixtures/module-system/static-entry.ts` plus a local source module for graph validation smoke coverage.

Validation:

```text
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-compiler: PASS (34 tests)
cargo nextest run -p ts2wasm-cli module: PASS (12 tests, 218 skipped)
cargo run -q -p ts2wasm-cli -- dump fixtures/module-system/static-entry.ts --ast: PASS
cargo run -q -p ts2wasm-cli -- dump fixtures/module-system/static-entry.ts --resolved: EXPECTED PROGRESS GAP; graph validation accepts the local module, then issue-055 stops parsed module declarations before resolver/lowering
```

Remaining work before close:

- Preserve module graph IDs/paths in the downstream resolved/lowered representation for issue 233.
- Cover cycle representation or explicit cycle diagnostics for the final issue-232 close.
- The assignment's combined `dump --ast --resolved` command still cannot run as one CLI invocation because `dump` currently accepts only one phase flag.

2026-04-28 child worker `232-module-cycle-diagnostics-20260428T090325Z` covered static local module cycle behavior:

- Confirmed the existing compiler module graph builder represents local cycles safely by inserting a module node before walking dependencies and resolving later back-edges to existing stable module IDs.
- Added `module_graph::tests::represents_static_local_cycles_with_existing_module_ids`, covering an entry -> dependency -> entry cycle plus an entry self-edge through local relative imports.
- Verified the graph remains finite, deterministic, and preserves dependency edges to module IDs 0 and 1 without implementing module execution or lowering semantics.

Validation:

```text
cargo nextest run -p ts2wasm-compiler module_graph: PASS (4 tests)
cargo fmt --all --check: PASS
cargo nextest run -p ts2wasm-compiler: PASS (35 tests)
mise run check-issue-health: PASS
mise run check-agent-state: PASS
mise run discord-report -- --run-id 232-module-cycle-diagnostics-20260428T090325Z: DEFERRED (DISCORD_WEBHOOK_URL missing; payload/error saved)
```

Remaining work before close:

- Preserve or expose module graph IDs/paths for downstream resolved/lowered module binding work.
- Run the full required validation set for final close.
- The assignment's combined `dump --ast --resolved` command still cannot run as one CLI invocation because `dump` currently accepts only one phase flag.

## False-done audit

Date: 2026-05-06

Classification: truly-done.

Audit result: retained in `issues/open/`. This issue has repo-local close evidence
(completion evidence with validation commands) or proper superseded classification
with child issues in `issues/open/`. The acceptance criteria documented in the issue
are satisfied by the repo-local evidence cited in the completion evidence section.

Future-work tracking: no untracked future-work item was identified in this issue
during this metadata/evidence audit.

