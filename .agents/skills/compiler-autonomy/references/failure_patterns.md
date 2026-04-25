# Failure pattern DB

This is a **bounded, curated** set of known failure classes. It replaces unbounded `llm_skills.md` dumps.

- Each pattern has a stable `FP-NNN` id.
- Prefer **short** entries; link to an issue, fixture path, or script if detail is long.
- When a pattern is hit in prod-like runs, the retro must add at least one **mechanical** guard: checklist line, fixture, script/lint, or gate — not prose alone.

## FP-001: Model JS `+` as numeric addition everywhere

**Trigger**

- `BinaryExpression` with operator `'+'`
- Operands are **not** statically proven to be `number`

**Check**

- String concatenation
- `ToPrimitive` / `ToString` paths
- `ToNumber` coercions
- Unknown or dynamic value kinds

**Required action**

- Lower only the number-proven fast path, or
- Emit an unsupported diagnostic, or
- Runtime dispatch (when in scope) — as defined by the task, not by convenience.

**Guards to add on recurrence**

- [ ] Item in `review_checklist.md` for `+` sites
- [ ] Regression fixture covering mixed-type or dynamic `+` if not already present
- [ ] If applicable, a gate script or type predicate assertion in IR lowering

## FP-002: Emit non-canonical capability manifest schema

**Trigger**

- Adding or modifying capability manifest emission in backend
- Changing `emit_manifest_v1_json` or related functions

**Check**

- Output schema matches `crates/shared/src/capability.rs::CapabilityManifest`
- Fields include `schema_version`, `target`, `standalone`, `wasi`, `node_host`, `capability_reasons`
- No transitional `ManifestV1` format with `imports/capabilities/runtime` arrays

**Required action**

- Use `CapabilityManifest` from `crates/shared` as the primary schema
- Map `RuntimeLinkPlan` to canonical fields via conversion function
- Update tests to validate canonical schema structure

**Guards to add on recurrence**

- [ ] Item in `review_checklist.md` to verify canonical schema fields
- [ ] Test fixture validates `schema_version`, `standalone`, `wasi.stdout`, `node_host.required`
- [ ] Gate script to check manifest JSON against canonical schema

## FP-003: Use array runtime for object computed property access

**Trigger**

- Adding or modifying property access lowering in resolver
- Handling `Expr::Index` or `ComputedIndex` in `builtin_resolver.rs`

**Check**

- String literal bracket notation `obj["key"]` uses `PropertyAccess` → `$property_get`
- Numeric bracket notation `arr[n]` uses `ComputedIndex` → `$array_get`
- Object tag check is used for string keys, array tag check for numeric keys

**Required action**

- Check index expression type in resolver before choosing runtime function
- Emit `PropertyAccess` for string literal indices, `ComputedIndex` otherwise
- Add regression fixture for computed property access on objects

**Guards to add on recurrence**

- [x] Item in `review_checklist.md` for bracket notation type discrimination
- [x] Regression fixture `fixtures/arrays-objects/computed-property.ts`
- [ ] Type predicate assertion in IR lowering for index expression kinds

_Add the next real incident as `FP-004+`; do not keep placeholder stubs._
