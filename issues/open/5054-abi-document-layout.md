---
id: 5054
title: "[runtime-abi] Document value tags and object layout as public ABI (audit reopened #5054)"
type: docs
area: abi
class: implementation-ready
priority: P2
depends_on: []
blocks: []
created: 2026-05-03
updated: 2026-05-05
---

## Summary

backend/runtime/shared が同じ前提を参照できるよう、value tag、heap object、array、BigInt、GC header の仕様を docs 化する。

## Problem

value tag、heap object layout、array/BigInt 表現、GC header が暗黙的な知識であり、backend/runtime/shared 間で共通理解が困難。

## Current failure

ABI の仕様変更時に他 crate との整合性を手動で確認する必要がある。

## Desired final state

value tag、heap object、array、BigInt、GC header のレイアウト仕様が `docs/` で文書化される。

## Scope

In scope:
- [x] value tag 一覧と定義
- [x] heap object layout 仕様
- [x] array/BigInt 表現
- [x] GC header 仕様

Out of scope:
- [x] runtime 実装の詳細 (out of scope)

## Affected paths

Expected:
- `docs/`
- `crates/runtime-abi/`

## Acceptance criteria

- [x] value tag が文書化される
- [x] 各 heap object の layout が文書化される
- [x] GC header が文書化される

## Validation

```sh
cargo fmt --all --check
```

## Docs / current-state / issue sync

Final-state docs:
- [x] updated: `docs/14-runtime-abi.md`
- [x] updated: `docs/00-docs-list.md` (existing entry already covers the scope)

Current state:
- [x] not affected

Follow-up issues:
- [x] none

## Reopened by audit

Date: 2026-05-05

Classification: acceptance-not-actually-met.

Reopen reason: no `## Completion evidence` section is present, so close evidence cannot be cited from the issue file.

Violated acceptance: the issue cannot provide repo-local close evidence for its checked acceptance criteria while it remains in this state. Acceptance checkboxes were reset for re-verification.

Evidence files:
- `issues/open/5054-abi-document-layout.md` (moved from open/ per close evidence)

Split follow-up: none created in this audit wave; this reopened issue remains the tracking item.

## Completion evidence

### Value tags
- `docs/14-runtime-abi.md` ## Tagged i32 value representation: tag encoding table (undefined/null/false/true/number/string/array/object) with TAG_MASK/HEAP_MASK
- Typed wrapper types section: `TaggedValue`, `HeapPtr`, `LocalRawValue`
- Source-of-truth: `crates/runtime-abi/src/value.rs`

### Heap object layouts
- **String**: ## Heap String Object section with length+UTF-8 bytes layout
- **Array**: ## Heap Array Object section with sparse-capable layout (length/capacity/presence_words/elements)
- **Object**: ## Heap Object Layout (current) section with property_count/flags/prototype_ptr/entries layout (added in this commit)
- **BigInt**: ## BigInt value representation section with sign/limb_count/limbs/decimal layout
- **Heap number**: ## Tagged i32 value representation section with sentinel+prototype+decimal layout
- **Closure**: ## Closure heap object ABI section with sentinel/code_id/capture_count/env_flags/capture slots (CLOSURE_SENTINEL = -2 added explicitly)

### GC header
- ## Heap Object Header Design section: flags_and_type + body_size_bytes + sweep_next + reserved
- GC kind constants table: GC_KIND_UNKNOWN/STRING/ARRAY/OBJECT/BIGINT with values
- ## GC Strategy section: mark/sweep/trigger points

### ABI versioning
- ## ABI Versioning section (added in this commit): ABI_VERSION = 1, golden snapshot test, backward compat policy

### Validation
- `cargo fmt --all --check` => pass
- `cargo nextest run --package ts2wasm-runtime-abi` => 30/30 pass (including `abi_layout_golden_snapshot` and `backward_compat_v1_archive_matches_current`)
