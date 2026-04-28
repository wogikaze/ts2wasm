# Cycle report: 20260428-020a-ir-design

## 状態

Issue 020a を完了。Generic JavaScript semantic IR の設計契約を `docs/13-ir-contracts.md` に追加した。

## 目的

020b の実装前に、JS semantics を backend から切り離すための HIR 命令セットと設計判断を明文化する。

## 実施内容

- `docs/13-ir-contracts.md` に `IR design` section を追加した。
- semantic instruction set として Value / Local / Conversion / Operator / Property / Call / Control / Metadata を定義した。
- `JsAdd` を単一 semantic op として保持すること、method receiver を落とさないこと、TypeScript type hints を metadata として扱うことを設計判断として記録した。
- issue 020a を done に移動し、親 issue 020 の 020a checklist を更新した。

## 判断と根拠

型 hint があっても observable JavaScript semantics を壊さないため、HIR では意味論命令を保持し、fast path は runtime guard または証明済み typed lowering で扱う設計にした。

## 検証

- PASS: `grep -A 30 "IR design" docs/13-ir-contracts.md`
- PASS: `cargo fmt --all --check`
- PASS: `scripts/manager update-issue-index --check`
- PASS: `scripts/manager check-issue-health`
- PASS: `scripts/manager check-repo-smoke`
- PASS: `cargo nextest run --no-fail-fast`（204 passed, 4 skipped）

## リスク

これは設計スライスであり、Rust enum / lowering / validation の実装は issue 020b / 020c に残る。

## 次にやるべきこと

Issue 020b で TypeScript AST から semantic IR へ lowering する最小実装に進む。

## 完了・追加

完了: issue 020a。追加: `docs/13-ir-contracts.md` の IR design section。
