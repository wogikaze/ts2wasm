# GC call-frame root fixture coverage report

**Run ID**: 20260428-221-gc-call-frame-roots
**Commit**: c33a50a80db6f002b30a641584bdb02d786cf194
**Date**: 2026-04-28
**Target**: issue 221 partial slice / GC call-frame fixture coverage

## 状態

- `c33a50a` で function/call-frame local root の回帰 fixture を追加した。
- `fixtures/core-semantics/gc-call-frame-root.ts` を Node differential 対象に登録した。
- 2500 回規模の repeated allocation で `iwasm` が out-of-bounds memory access になる別問題を確認し、issue 222 として分離した。
- issue 221 は open のまま維持した。現時点の root table は保守的/static であり、nested activation frame の precise register/unregister と closure capture semantics は未完了。

## 実施内容

- `fixtures/core-semantics/gc-call-frame-root.ts` を追加し、関数内の live object が GC pressure 後も保持されることを検証対象にした。
- `crates/cli/tests/m2_node_diff.rs` の curated semantic fixture list に call-frame GC fixture を追加した。
- `fixtures/core-semantics/gc-object-root.ts` と新 fixture の iteration を 2000 に揃え、現時点で再現可能な安定ゲートにした。
- 高圧 allocation OOB は fixture/root registration 完了判定と混ぜず、`issues/open/222-investigate-gc-high-pressure-oob.md` に追跡を切り出した。

## 判断

- このコミットは「call-frame root fixture coverage」の前進であり、issue 221 全体の完了ではない。
- closure capture は lowering 側でまだ未実装で、issue 210 のスコープに残っている。
- repeated allocation OOB は GC root registration ではなく allocator/sweep/free-list pressure の不具合として扱う。

## 検証

- `cargo fmt --all --check`: pass
- `scripts/manager update-issue-index --check`: pass
- `scripts/manager check-issue-health`: pass
- `scripts/manager check-repo-smoke`: pass
- `cargo nextest run -p ts2wasm-cli --test m2_node_diff m3_semantic_fixtures_match_node_output_under_iwasm`: pass, 1 passed / 19 skipped
- `cargo nextest run --no-fail-fast`: pass, 230 passed / 4 skipped

## 次のタスク

- issue 222: high-pressure local-root allocation の `iwasm` OOB を最小再現し、allocator/sweep/free-list の破損箇所を修正する。
