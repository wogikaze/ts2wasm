# Cycle Report: issue 235

Run id: `235-gc-root-count-20260428T055606Z`
Branch: `agent/235-gc-root-count-20260428T055606Z`
Implementation commit: `0090300`

## 状態

DONE. Issue 235 was fixed, validated, moved to `issues/done/`, and the issue
index was regenerated.

## 目的

Restore the two backend GC-root tests that failed after backend local-frame
sizing changed.

## 実施内容

- Updated top-level GC root count/allocation assertions to derive values from
  `LocalFrame`.
- Updated function activation-root assertions to derive static root bytes,
  call-frame offsets, and frame byte counts from `LocalFrame`/`Layout`.
- Preserved explicit mirror assertions for user locals and backend temporaries.
- Recorded completion evidence in issue 235 and regenerated `issues/index.md`.

## 判断と根拠

The implementation was correct and the stale expectations were wrong. The
emitter mirrors `LocalFrame::total_local_count()` roots, and the current
backend frame has 12 backend temporaries.

Top-level evidence: one user local plus 12 backend temporaries gives
`$gc_root_count = 13` and a 52-byte root allocation.

Function evidence: `_start` reserves 12 static backend roots, so
`$gc_root_count = 12`, `$gc_call_frame_base` starts at byte offset 48, and the
reserved call-frame stack makes the root allocation 16432 bytes.

## 詰まり・ロス

Discord reporting is deferred because `DISCORD_WEBHOOK_URL` is not configured
in the environment or `.env`. Payload and error artifacts were saved under this
run directory.

## リスク

None known. Full workspace validation passed.

## 次にやるべきこと

Merge the child branch after parent review.

## 完了 / 追加

Completed: issue 235. Follow-up issues: none.

## Validation

```text
command: cargo fmt --all --check
result: PASS

command: cargo nextest run -p ts2wasm-backend-wasm function_locals_are_mirrored_into_activation_gc_root_frames top_level_locals_are_mirrored_into_gc_root_table
result: PASS (2 passed)

command: cargo nextest run -p ts2wasm-backend-wasm
result: PASS (15 passed)

command: scripts/manager update-issue-index && scripts/manager check-issue-index
result: PASS

command: scripts/manager check-issue-health
result: PASS

command: scripts/manager check-agent-state
result: PASS

command: cargo nextest run
result: PASS (339 passed, 4 skipped)
```
