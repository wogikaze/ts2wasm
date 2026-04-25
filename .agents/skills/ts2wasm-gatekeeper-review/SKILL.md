---
name: ts2wasm-gatekeeper-review
description: "Use when acting as a gatekeeper/reviewer for ts2wasm PRs, agent outputs, or autopilot results before mainline merge. Trigger phrases: review as gatekeeper, merge gate check, runtime/backend checklist, grep gate, handoff packet."
---

# ts2wasm Gatekeeper Review Workflow

Use this skill when deciding whether a change is safe to merge into mainline.

Primary source of truth:
- docs/11-shared-definitions.md（workstreams、gates、test status schema、capability / benchmark policy）
- docs/13-coding-standard.md, section 19 (Gatekeeper Checklist)
- docs/13-coding-standard.md, section 20 (priority context)
- current-state.md（実装の現在地と代表コマンド）

## Goal

Produce a merge decision based on explicit gates, not intuition.

- Pass: all required gates are satisfied, residual risks are documented.
- Reject: any immediate reject condition is present.
- Hold: missing evidence (tests/docs/handoff) prevents decision.

## 1) Immediate Reject Scan

Reject immediately if any of these appear:

- Parser judges builtin/API/host capability.
- Resolver/Lowering knows host imports.
- Backend does name resolution/builtin discovery/arity checks.
- RuntimeFn/HostImport/Capability additions without required catalog + tests + docs.
- Runtime strings are unconditional.
- fd_write/fd_read are always imported.
- New source-origin diagnostics with span: None.
- Docs gate is removed to make progress look complete.
- Gate / workstream 進捗が実装・テスト・artifact なしで doc-only になっている。

## 2) First Command Set (always run)

Run these first:

- git status --short
- git log --oneline -8
- git diff --stat HEAD~1..HEAD
- git show --name-only --oneline HEAD
- cargo fmt --all --check
- cargo nextest run

If runtime/backend/WASI/differential changed, also run one of:

- cargo nextest run（該当 filterset / パッケージがあればそれを使用）
- project-defined iwasm differential suite

## 3) Grep Gate (regression traps)

Run（検索は `ig` を優先。未導入なら `rg`。他 skill と同様に `ig -n '<pattern>' <path>` 形式）:

- ig -n 'as_console_log_call' crates/cli/src
- ig -n 'property == "length"' crates/cli/src/ir/lowered.rs
- ig -n 'fd_write|fd_read' crates/cli/src/backend
- ig -n 'RuntimeString::.*intern|intern_required_runtime_strings' crates/cli/src/backend
- ig -n 'span: None' crates/cli/src
- ig -n 'unwrap\(|expect\(|panic!' crates/cli/src

Default judgment:

- Any non-test compiler-path unwrap/expect/panic: reject unless justified with explicit comment.
- fd_write/fd_read hardcoded outside RuntimeLinkPlan flow: reject.
- New source diagnostic span: None: reject.

## 4) Domain Checklists

Apply only those relevant to touched scope.

### RuntimeFn addition

Must include:
- RuntimeFn variant + RuntimeSpec + deps/imports/capability/runtime_strings/result.
- emission_order + all updates.
- runtime_builder emit function.
- RuntimeLinkPlan tests.
- capability manifest tests.
- differential tests if behavior changed.
- current-state.md and docs/15 updates.

### HostImport/Capability addition

Must include:
- enums and RuntimeSpec wiring.
- RuntimeLinkPlan required_imports/required_capabilities tests.
- manifest JSON tests.
- conditional WAT import present/absent tests.
- docs/09 and current-state.md updates.

### Builtin addition

Must include:
- ir/builtin contract.
- BuiltinResolver source pattern.
- arity/result contract.
- unsupported-args diagnostics and negative tests.
- Lowering handling explicitly stated.
- runtime mapping or compile-time gate.
- docs/05 or current-state.md subset semantics update.

### Memory layout changes

Must include:
- docs/15 consistency.
- validate_memory_layout updates for every new fixed region.
- ordered inequalities enforced:
  - static data end <= SCRATCH_OFFSET
  - SCRATCH_OFFSET + SCRATCH_SIZE <= next fixed buffer
  - fixed buffer end <= HEAP_START
- alignment consistency with ValueTag::HEAP_MASK.
- large allocation/OOM policy documented.

## 5) RuntimeLinkPlan Gate

Require all of these:
- RuntimeLinkPlan collects required_runtime.
- imports/capabilities/runtime_strings derived from RuntimeSpec.
- WatEmitter does not compute dependency closure by hand.
- manifest generation comes from RuntimeLinkPlan (not ad-hoc rescans).

## 6) Docs Gate

If semantics/ABI/layout/capability/unsupported set changed, docs must be updated in the same slice.
Minimum candidates:
- docs/05-compatibility-and-semantics.md
- docs/09-security-and-capability-model.md
- docs/11-shared-definitions.md
- current-state.md
- docs/15-runtime-abi.md

## 7) Output Contract (what reviewer must deliver)

Always output in this order:

1. Findings first (highest severity first), with concrete file/line references.
2. Open assumptions/questions (only if decision depends on missing evidence).
3. Merge decision: pass/reject/hold.
4. Gate handoff packet.

## 8) Gate Handoff Template

Use this exact structure:

Gate handoff

Scope:
- Implemented:
- Not implemented:

Commits:
- <hash> <title>
- <hash> <title>

Validation:
- cargo fmt --all --check: pass/fail
- cargo nextest run: pass/fail
- scripts/update_coverage_matrix.sh --check: pass/fail/not applicable
- iwasm differential: pass/fail/not applicable
- grep gate:
  - as_console_log_call: 0/non-zero
  - property == "length" in lowered.rs: 0/non-zero
  - source diagnostic span None added: no/yes

Risk:
- Runtime behavior changed: yes/no
- Host import changed: yes/no
- Memory layout changed: yes/no
- Manifest schema changed: yes/no

Docs:
- current-state.md updated: yes/no
- docs/15 updated if ABI changed: yes/no
- docs/09 updated if capability changed: yes/no

Known unrelated working tree changes:
- <file>

## 9) Review Mindset

- Be strict on phase separation and linker ownership.
- Prefer rejecting incomplete slices over merging hidden debt.
- Do not mark gate / workstream progress as done unless implementation + tests + docs + generated artifacts (where applicable) all exist.
