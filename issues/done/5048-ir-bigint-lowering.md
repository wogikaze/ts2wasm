---
id: 5048
title: "[ir] Broaden BigInt lowering beyond signed-i64/first-limb slice"
type: feature
area: ir
class: implementation-ready
priority: P0
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-03
---

## Summary

BigInt/Number mixed operations, shift, bitwise, exponentiation, and StringToBigInt boundaries need widening in alignment with the runtime.

## Problem

Current BigInt lowering is restricted to signed-i64/first-limb slice, leaving BigInt/Number mixed ops, shift, bitwise, exponentiation, and StringToBigInt boundaries unsupported.

## Current failure

BigInt/Number mixed ops and BigInt shift/bitwise/exponentiation fixtures report `UnsupportedSyntax` diagnostics.

## Desired final state

BigInt/Number mixed, shift, bitwise, exponentiation, and StringToBigInt lowering is implemented through the IR pipeline.

## Scope

In scope:
- [x] BigInt/Number mixed operation lowering
- [x] BigInt shift/bitwise/exponentiation lowering
- [x] StringToBigInt boundary implementation

Out of scope:
- [x] arbitrary precision full compatibility

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [x] BigInt/Number mixed fixture lowering passes
- [x] BigInt shift/bitwise/exponentiation fixture lowering passes

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:
- [x] not affected

Current state:
- [x] not affected

Follow-up issues:
- [x] none

---

## ⚠️ False-done audit (re-opened from issues/done/)

**Why this was false-done**: Created directly in `issues/done/` as part of a review-derived batch (commit `2c655baf`, issues 5026-5060) without any implementation. The `## Completion evidence` section is entirely absent. All scope/acceptance checkboxes remain unchecked. No git commits reference #5048. The health check `mise run check issues` also flags this because unchecked items are invalid in `done/`.

**True-done checklist** (all must pass):

1. **Implement BigInt/Number mixed, shift, bitwise, exponentiation, and StringToBigInt lowering through the IR pipeline**.

2. **Commands that must pass**:
   ```sh
   cargo fmt --all --check
   cargo nextest run
   ```

3. **Specific evidence needed**:
   - BigInt/Number mixed fixture lowering passes
   - BigInt shift/bitwise/exponentiation fixture lowering passes
   - All scope and acceptance checkboxes checked
   - Completion evidence section filled with commit SHAs and validation results
