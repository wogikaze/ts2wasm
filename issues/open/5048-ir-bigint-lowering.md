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
- [ ] BigInt/Number mixed operation lowering
- [ ] BigInt shift/bitwise/exponentiation lowering
- [ ] StringToBigInt boundary implementation

Out of scope:
- [ ] arbitrary precision full compatibility

## Affected paths

Expected:
- `crates/ir/src/`

## Acceptance criteria

- [ ] BigInt/Number mixed fixture lowering passes
- [ ] BigInt shift/bitwise/exponentiation fixture lowering passes

## Validation

```sh
cargo fmt --all --check
cargo nextest run
```

## Docs / current-state / issue sync

Final-state docs:
- [ ] not affected

Current state:
- [ ] not affected

Follow-up issues:
- [ ] none
