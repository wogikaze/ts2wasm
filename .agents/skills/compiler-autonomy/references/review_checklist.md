# Self-review gate checklist

**Before SELF_REVIEW_GATE → VERIFY_FAST**

## Scope and drift

- [ ] Changes are limited to `current_task.json` → `scope.allowed_files` (or explicitly updated scope with rationale in `decision_log.md`)
- [ ] No silent edits to forbidden docs or out-of-scope crates in the same change set
- [ ] `docs/` edits are only present if the task is docs-scoped, or a separate docs issue is linked

## Semantics and compatibility

- [ ] Language compatibility decisions match `docs/05-compatibility-and-semantics.md` (or a noted deviation filed as a follow-up issue)
- [ ] `+` and other double-edged operators: number-only when claimed (see this skill’s `references/failure_patterns.md` FP-001)
- [ ] Bracket notation `obj[key]` discriminates string literal vs numeric: string uses `PropertyAccess`, numeric uses `ComputedIndex` (see FP-003)

## Quality bar

- [ ] Formatting: `cargo fmt --all --check` will pass
- [ ] New behavior has a fixture or test hook named in acceptance when applicable
- [ ] No `#[ignore]` or skip added to make red tests green (unless a task explicitly authorizes a targeted skip and records follow-up work)

## Outputs

- [ ] Issue (if any) can record exact commands to reproduce verification
- [ ] `reports/runs/<run_id>/test_report.json` will be produced (or a recorded reason not to) before closing

## Capability manifest schema

- [ ] Capability manifest uses canonical schema from `crates/shared/src/capability.rs::CapabilityManifest`
- [ ] Output includes `schema_version`, `target`, `standalone`, `wasi`, `node_host`, `capability_reasons`
- [ ] No transitional `ManifestV1` format with `imports/capabilities/runtime` arrays
