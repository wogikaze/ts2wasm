---
id: 304
title: "Support ABC451 depth-8 live-set after memory policy"
type: feature
area: runtime/memory
class: implementation-ready
priority: P1
depends_on: []
blocks: [300,294]
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Implement the next smaller ABC451 runtime allocation/memory slice after issue
303. The depth-7 search reducer now passes under the 42-page cap, but depth 8
still traps in `$alloc_heap` even though the same reducer passes with a larger
temporary memory maximum.

This issue owns only the depth-8 reducer. The official ABC451 depth-9 sample
path remains owned by issue 300 or a later child after this slice is solved.

## Problem

The current runtime cannot hold the ABC451 depth-8 recursive search live set
under the committed 42-page memory policy.

Problem: ABC451 depth-8 reducer prints Node `292743`, but committed iwasm traps
with `Exception: unreachable`; a temporary 256-page cap prints `292743`.

## Current failure

Reduced repro:

```sh
cat > /tmp/abc451-search-depth-8.ts <<'TS'
function search(before: string, powersOfTwoStr: string[]): string[] {
    const answers: string[] = [];
    if (before.length > 0) answers.push(before);
    const remainDigits = 8 - before.length;
    for (let i = 0; i < powersOfTwoStr.length; i++) {
        const after = powersOfTwoStr[i];
        if (after.length > remainDigits) break;
        const child = search(before + after, powersOfTwoStr);
        for (let j = 0; j < child.length; j++) {
            answers.push(child[j]);
        }
    }
    return answers;
}
const powersOfTwo: number[] = [];
for (let i = 0; 2 ** i <= 1000000000; i++) {
    powersOfTwo.push(2 ** i);
}
const powersOfTwoStr: string[] = powersOfTwo.map(n => String(n));
const allGoodIntStr = search("", powersOfTwoStr);
console.log(allGoodIntStr.length);
TS

node /tmp/abc451-search-depth-8.ts
cargo run -q -- build /tmp/abc451-search-depth-8.ts -o /tmp/abc451-search-depth-8.wasm --host-deny
iwasm /tmp/abc451-search-depth-8.wasm
```

Current result:

```text
node: 292743
iwasm with committed MEMORY_MAX_PAGES=42: Exception: unreachable
```

Temporary cap evidence:

```sh
cargo run -q -- dump --wat /tmp/abc451-search-depth-8.ts > /tmp/abc451-depth-8.wat
tail -n +2 /tmp/abc451-depth-8.wat > /tmp/abc451-depth-8.clean.wat
sed 's/(memory (export "memory") 2 42)/(memory (export "memory") 2 128)/' \
  /tmp/abc451-depth-8.clean.wat > /tmp/abc451-depth-8-cap-128.wat
wat2wasm /tmp/abc451-depth-8-cap-128.wat -o /tmp/abc451-depth-8-cap-128.wasm
iwasm /tmp/abc451-depth-8-cap-128.wasm
sed 's/(memory (export "memory") 2 42)/(memory (export "memory") 2 256)/' \
  /tmp/abc451-depth-8.clean.wat > /tmp/abc451-depth-8-cap-256.wat
wat2wasm /tmp/abc451-depth-8-cap-256.wat -o /tmp/abc451-depth-8-cap-256.wasm
iwasm /tmp/abc451-depth-8-cap-256.wasm
```

Observed result on 2026-04-29:

```text
cap 128: Exception: unreachable
cap 256: 292743
```

Nearby controls:

```text
depth 7: Node/iwasm both print 61002 under committed 42-page cap
depth 9: Node prints 1404832; temporary 512/1024-page iwasm trials did not
         finish within 90 seconds during issue-300 triage
```

## Desired final state

The depth-8 ABC451 search reducer prints Node-matching `292743` under the
committed default runtime configuration while the OOM boundary remains explicit
and tested.

## Scope

In scope:

- [ ] Support the exact depth-8 recursive search live-set reducer under iwasm.
- [ ] Choose the smallest architecture-preserving implementation path:
      documented memory-policy adjustment, reduced temporary retention, or a
      compact allocation/representation change.
- [ ] Preserve the intentional OOM regression boundary.
- [ ] Keep the existing integer-only heap-number subset unchanged.

Out of scope:

- Full IEEE-754 `number` semantics, fractional values, `NaN`, `Infinity`, or
  `-0`.
- BigInt runtime representation.
- Problem-specific ABC451 source rewrites or generated replacement algorithms.
- Claiming official ABC451 depth-9 sample compatibility.

## Affected paths

Expected:

- `crates/runtime-abi/src/layout.rs`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `docs/14-runtime-abi.md` if runtime memory policy/layout changes
- `current-state.md` if supported live-set behavior changes
- `issues/open/300-support-abc451-large-integer-number-boundary.md`
- `issues/index.md`

Do not touch:

- problem-specific source rewrite hooks
- BigInt runtime/ABI files
- issue 302 eval files

## Acceptance criteria

- [ ] A committed regression fixture or scripted validation proves the depth-8
      reducer prints `292743` under the default emitted wasm memory policy.
- [ ] Existing depth-7 reducer evidence remains valid.
- [ ] Existing intentional OOM coverage still traps under the chosen policy.
- [ ] Issue 300 is updated with the depth-8 result and the next blocker.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
cargo nextest run -p ts2wasm-cli <focused-new-or-affected-tests>
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo run -q -- build /tmp/abc451-search-depth-8.ts -o /tmp/abc451-search-depth-8.wasm --host-deny
iwasm /tmp/abc451-search-depth-8.wasm
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-post-depth8.wasm --host-deny
printf '10\n' | iwasm /tmp/abc451-d-post-depth8.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/14-runtime-abi.md` if runtime memory policy/layout changes

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` if supported behavior changes

Follow-up issues:

- [ ] none
- [ ] created/updated if depth 9 or official samples expose a distinct next
      runtime allocation/GC/representation blocker.

## Notes

During issue-300 triage, clearing the narrow `ArrayPushGrow` backend temporary
roots after a push did not change the reducer threshold: depth 8 still failed
at 64/128 pages and passed at 256 pages. That experiment was not committed.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
