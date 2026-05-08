---
id: 303
title: "Define runtime memory limit policy for large live sets"
type: feature
area: runtime/memory
class: done
priority: P1
depends_on: []
blocks: [300]
created: 2026-04-29
updated: 2026-04-29
status: done
completed: 2026-04-29
---

## Summary

Define and implement the runtime memory limit policy needed for large but valid
standalone workloads such as the ABC451 D search fixture.

Problem: Current wasm modules declare a hard maximum of 16 pages (1 MiB), which
is too small for ABC451-style live result arrays even after allocator free-list
reuse bugs are fixed.

## Current failure

Reduced ABC451 live-set reproducer:

```sh
cat > /tmp/abc451-search-depth-7.ts <<'TS'
function search(before: string, powersOfTwoStr: string[]): string[] {
    const answers: string[] = [];
    if (before.length > 0) answers.push(before);
    const remainDigits = 7 - before.length;
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

node /tmp/abc451-search-depth-7.ts
cargo run -q -- build /tmp/abc451-search-depth-7.ts -o /tmp/abc451-search-depth-7.wasm --host-deny
iwasm /tmp/abc451-search-depth-7.wasm
```

Evidence from issue 300 child run:

```text
node: 61002
iwasm with committed MEMORY_MAX_PAGES=16: Exception: unreachable
iwasm with only a temporary MEMORY_MAX_PAGES=512 change: 61002
```

Nearby control:

```text
depth 6 reducer: node=12711, iwasm=12711 under committed 16-page cap
```

ABC451 full sample remains blocked by the same policy class:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-large-number-child.wasm --host-deny
printf '10\n' | iwasm /tmp/abc451-d-large-number-child.wasm
```

Current result:

```text
Exception: unreachable
```

## Desired final state

The runtime has a documented memory limit policy that lets large standalone
fixtures grow enough memory for valid live sets while retaining an explicit,
tested OOM boundary.

## Scope

In scope:

- [x] Choose and document the default wasm memory maximum for standalone
      modules.
- [x] Update `Layout::MEMORY_MAX_PAGES` or add an equivalent compiler/runtime
      policy knob.
- [x] Update OOM regression coverage so intentional OOM tests still exceed the
      new policy.
- [x] Verify the depth-7 reducer prints `61002` under `iwasm`.
- [x] Re-run the ABC451 sample commands and record whether they advance.

Out of scope:

- Full IEEE-754 number semantics.
- BigInt runtime representation.
- Problem-specific ABC451 source rewrites.
- Replacing the user program with a generated algorithm.

## Affected paths

Expected:

- `crates/runtime-abi/src/layout.rs`
- `crates/backend-wasm/src/`
- `crates/cli/tests/`
- `fixtures/core-semantics/`
- `fixtures/atcoder/`
- `docs/14-runtime-abi.md`
- `docs/05-compatibility-and-semantics.md` only if user-visible runtime limits
  are described there
- `issues/done/300-support-abc451-large-integer-number-boundary.md`
- `issues/index.md`

Do not touch:

- problem-specific source rewrite hooks
- unrelated number semantics

## Acceptance criteria

- [x] A committed regression or scripted validation proves the depth-7 reducer
      prints `61002` under `iwasm`.
- [x] Existing OOM coverage still traps intentionally under the chosen policy.
- [x] `fixtures/atcoder/abc451-d-concat-power2.ts` is rebuilt and the three
      official sample commands are re-run, with pass output or a newly narrowed
      blocker recorded in issue 300.
- [x] Runtime memory-limit documentation/current-state is synchronized if the
      default maximum changes.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
cargo nextest run -p ts2wasm-cli
mise run update-issue-index -- --check
mise run check issues
```

Impacted commands:

```sh
cargo run -q -- build /tmp/abc451-search-depth-7.ts -o /tmp/abc451-search-depth-7.wasm --host-deny
iwasm /tmp/abc451-search-depth-7.wasm
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-large-number-child.wasm --host-deny
printf '10\n' | iwasm /tmp/abc451-d-large-number-child.wasm
printf '69\n' | iwasm /tmp/abc451-d-large-number-child.wasm
printf '1099898\n' | iwasm /tmp/abc451-d-large-number-child.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [x] updated: `docs/14-runtime-abi.md` if the default wasm memory maximum changes

Current state:

- [x] updated: `current-state.md` if supported standalone workload limits change

Follow-up issues:

- [x] none

## Notes

The reducer proves the allocator/GC path can produce the expected result when
the only change is a higher memory maximum. This issue should preserve an
explicit OOM boundary rather than simply removing the maximum.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `b4209d4` (`issue-303: define runtime memory cap`)

Validation result:

```text
command: cargo run -q -- build /tmp/abc451-search-depth-7.ts -o /tmp/abc451-search-depth-7.wasm --host-deny && iwasm /tmp/abc451-search-depth-7.wasm
result: pass; iwasm stdout `61002`
date: 2026-04-29

command: cargo nextest run -p ts2wasm-cli oom_alloc_check_must_fail_iwasm
result: pass; intentional OOM fixture still traps under the 42-page cap
date: 2026-04-29

command: cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d-memory-policy-child.wasm --host-deny
result: pass
date: 2026-04-29

command: printf '10\n' | iwasm /tmp/abc451-d-memory-policy-child.wasm
result: blocked; `Exception: unreachable`
date: 2026-04-29

command: printf '69\n' | iwasm /tmp/abc451-d-memory-policy-child.wasm
result: blocked; `Exception: unreachable`
date: 2026-04-29

command: printf '1099898\n' | iwasm /tmp/abc451-d-memory-policy-child.wasm
result: blocked; `Exception: unreachable`
date: 2026-04-29
```

Remaining risks:

- The full ABC451 official sample path still traps in runtime allocation under
  the new cap; issue 300 remains open with the recorded blocker.

## False-done audit

Date: 2026-05-05

Classification: truly-done.

Audit result: retained in `issues/done/`. The issue has repo-local completion evidence, is not marked `blocked` or `triage-needed`, and `python scripts/manager.py check issues` validates the resulting issue graph.

Evidence files:
- `issues/done/303-define-runtime-memory-limit-policy-for-large-live-sets.md`
- `issues/index.md` after regeneration

Future-work tracking: no untracked future-work item was identified in this issue during this metadata/evidence audit.
