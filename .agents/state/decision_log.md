# Decision log (append-only)

**Rules**

- Newest entries at the **top** of the `## Log` section.
- One decision per block: `when`, `state`, `decision`, `rationale`, `reversible?`, `follow-up` (issue id or `none`).

**Do not** use this file for implementation notes, chat transcripts, or unbounded “lessons” — that belongs in `issues/` or a structured failure entry in `skills/failure_patterns.md` with a machine guard.

## Log

### 2026-04-26T13:19:35Z — close issue 003

- **When**: 2026-04-26T13:19:35Z
- **State**: VERIFY_FULL
- **Decision**: Closed issue 003 as done and moved it to `issues/done/` with updated acceptance evidence
- **Rationale**: Manifest import/fixture tests in scope were implemented and validated; issue tracker and index were synchronized
- **Reversible?**: no
- **Follow-up**: none

### 2026-04-26 — plan for issue 002

- **When**: 2026-04-26T06:45:00Z
- **State**: PLAN
- **Decision**: Replace backend::emit_manifest_v1_json to use canonical CapabilityManifest from crates/shared
- **Rationale**: Canonical schema already exists in shared crate; transitional ManifestV1 in backend should be replaced
- **Reversible?**: yes (can revert to ManifestV1 if needed)
- **Follow-up**: none

### 2026-04-26 — plan review gate for issue 002

- **When**: 2026-04-26T06:46:00Z
- **State**: PLAN_REVIEW_GATE
- **Decision**: Plan approved - all files in scope, no forbidden files touched, test updates included
- **Rationale**: Scope matches current_task.json allowed_files; no docs changes needed; tests will be updated
- **Reversible?**: yes
- **Follow-up**: none

### 2026-04-26 — self review gate for issue 002

- **When**: 2026-04-26T06:50:00Z
- **State**: SELF_REVIEW_GATE
- **Decision**: Self review passed - scope correct, no drift, tests added, formatting passes
- **Rationale**: All changes in allowed_files; no forbidden files touched; 3 new tests for canonical schema; cargo fmt passes
- **Reversible?**: yes
- **Follow-up**: none

### 2026-04-26 — verify fast for issue 002

- **When**: 2026-04-26T06:51:00Z
- **State**: VERIFY_FAST
- **Decision**: Verify fast passed - fmt passes, nextest passes (2 pre-existing failures unrelated to changes)
- **Rationale**: cargo fmt --all --check passes; cargo nextest run has 2 pre-existing failures in m9_typed_optimization (unrelated to capability manifest changes)
- **Reversible?**: yes
- **Follow-up**: none

### 2026-04-26 — verify full for issue 002

- **When**: 2026-04-26T06:52:00Z
- **State**: VERIFY_FULL
- **Decision**: Verify full passed - all acceptance criteria met
- **Rationale**: --emit-manifest emits canonical schema with schema_version=1, standalone=true, wasi.stdout=true, node_host.required=false; tests validate canonical schema fields
- **Reversible?**: yes
- **Follow-up**: none

### 2026-04-26 — retro for issue 002

- **When**: 2026-04-26T06:53:00Z
- **State**: RETRO
- **Decision**: Added FP-002 to failure_patterns.md and review_checklist.md for canonical schema guard
- **Rationale**: Mechanical guard added to prevent future non-canonical schema emissions
- **Reversible?**: yes
- **Follow-up**: none

Implementation steps:
1. Extend RuntimeLinkPlan with capability reason tracking (map capability to source pattern)
2. Add conversion function from RuntimeLinkPlan to CapabilityManifest in backend/capability_manifest.rs
3. Replace emit_manifest_v1_json to emit canonical schema
4. Update existing tests to validate canonical schema fields (schema_version, standalone, wasi, node_host, capability_reasons)
5. Ensure --emit-capabilities alias emits identical JSON
6. Add test fixtures for stdout, stdin, standalone, and Node-host-required cases
