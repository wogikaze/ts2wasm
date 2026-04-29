---
id: 304
title: "Support ABC451 depth-8 live-set after memory policy"
type: feature
area: runtime/memory
class: done
priority: P1
depends_on: []
blocks: [300,294]
created: 2026-04-29
updated: 2026-04-29
status: done
completed: 2026-04-29
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

- [x] Support the exact depth-8 recursive search live-set reducer under iwasm.
- [x] Choose the smallest architecture-preserving implementation path:
      documented memory-policy adjustment, reduced temporary retention, or a
      compact allocation/representation change.
- [x] Preserve the intentional OOM regression boundary.
- [x] Keep the existing integer-only heap-number subset unchanged.

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

- [x] A committed regression fixture or scripted validation proves the depth-8
      reducer prints `292743` under the default emitted wasm memory policy.
- [x] Existing depth-7 reducer evidence remains valid.
- [x] Existing intentional OOM coverage still traps under the chosen policy.
- [x] Issue 300 is updated with the depth-8 result and the next blocker.

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

- [x] updated: `docs/14-runtime-abi.md` for runtime memory policy change

Current state:

- [x] updated: `current-state.md` for supported live-set behavior change

Follow-up issues:

- [x] updated: issue 300 remains the depth-9 / official sample follow-up
      runtime allocation/GC/representation blocker.

## Notes

During issue-300 triage, clearing the narrow `ArrayPushGrow` backend temporary
roots after a push did not change the reducer threshold: depth 8 still failed
at 64/128 pages and passed at 256 pages. That experiment was not committed.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `423feb4f` (`issue-304: support depth-8 live set`)

Validation result:

```text
command: cargo run -q -- build /tmp/abc451-search-depth-8.ts -o /tmp/abc451-search-depth-8.wasm --host-deny && iwasm /tmp/abc451-search-depth-8.wasm
result: pass; iwasm stdout `292743`
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli abc451_depth8_live_set_fixture_matches_node_output_under_iwasm
result: pass
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass; generated fast OOM fixture traps under the 185-page cap
date: 2026-04-29

command: cargo fmt --all --check
result: pass
date: 2026-04-29

command: mise run update-issue-index -- --check
result: pass; issues/index.md OK
date: 2026-04-29

command: mise run check issues
result: unrelated pre-existing failure; issue 304 is clean, remaining errors are missing test262 result artifact references in issues 289, 292, 271, 284, 285, 286, 288, 291, 293, and 296
date: 2026-04-29

command: cargo run -q -- build /tmp/abc451-search-depth-7.ts -o /tmp/abc451-search-depth-7.wasm --host-deny && iwasm /tmp/abc451-search-depth-7.wasm
result: pass; iwasm stdout `61002`
date: 2026-04-29
```

Remaining risks:

- Official depth-9 ABC451 sample compatibility remains unclaimed and tracked by issue 300.
