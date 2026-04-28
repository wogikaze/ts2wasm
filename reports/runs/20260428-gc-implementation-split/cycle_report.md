# Cycle report: 20260428-gc-implementation-split

## 状態

Issue 017b は scope が大きすぎるため、tracking issue に戻して 217/218/219 の実装 slice に分割した。

## 目的

GC 実装を header/accounting、mark root scanning、sweep/reuse/fixtures の順に検証可能な単位へ分け、次の実装作業を issue 217 として開始できる状態にする。

## 実施内容

- 017b を `blocked` の tracking issue に変更した。
- 217: GC heap header と allocation trigger accounting を追加した。
- 218: mark phase root scanning を追加した。
- 219: sweep/free-list reuse と GC fixtures を追加した。
- `issues/index.md` を再生成した。

## 判断と根拠

017b の元 acceptance は header、mark/sweep、trigger、fixtures を同時に要求していた。GC は runtime ABI と backend allocation path にまたがるため、一括実装よりも段階的に validation できる issue 分割が安全。

## 検証

- PASS: `scripts/manager update-issue-index`
- PASS: `scripts/manager update-issue-index --check`
- PASS: `scripts/manager check-issue-health`
- PASS: `git diff --check`

## リスク

017b 自体はまだ完了していない。次は issue 217 で `$alloc_heap` の payload ABI を維持しながら GC header と trigger accounting を実装する。

## 次にやるべきこと

Issue 217 を実装し、runtime ABI constants と backend WAT emission の contract test を追加する。

## 完了・追加

完了: 017b の分割。追加: issue 217、218、219。
