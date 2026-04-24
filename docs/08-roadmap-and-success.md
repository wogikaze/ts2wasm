# Roadmap and success conditions

この文書は実装ロードマップと成功条件を扱う。milestone の canonical 定義は `docs/11-shared-definitions.md` に置き、この文書では実装順序と成功条件の読み方を説明する。

## 実装ロードマップ

最初の段階では、TypeScript の小さなプログラムを `.wasm` に変換し、stdio で結果を出せる状態を作る。ここでは `let`、`const`、数値、文字列、boolean、if、while、function、array、object literal の基本を扱う。出力は WASI `.wasm` とし、iwasm で動作することを重視する。

次の段階では、compile-time evaluator を捨て、JS 値 runtime を WASM 側で動かす。M4 はこの移行ゲートであり、`undefined`、`null`、truthiness、`===`、`+` の M3 fixtures を、事前計算した stdout ではなく WASM runtime 上の JS value execution で通す。M4 を通るまでは、M5 以降の機能追加を進めない。

その次に、TypeScript の型情報を pipeline に取り込む。型による fast path、診断、unsupported feature の分類を行う。`tsc` parser/checker との差分を取り、公式挙動とのズレを管理する。

中期では、module、class、exception、JSON、fs、process、Buffer、path を広げる。Node host 併用ターゲットを整え、WASI だけで動くものと Node host が必要なものを明確に分ける。特に stdin は Node.js に丸投げしない。`fs.readFileSync(0, "utf8")` は WASI `fd_read` と WASM runtime の UTF-8/string 処理へ lowering する。

後期では、test262 / TypeScript tests の coverage dashboard を整備し、performance dashboard を継続的に更新する。Wasm GC backend、Component Model / WIT backend、より強い最適化、shape cache、inline cache、typed array 最適化などを追加する。

## 成功条件

このプロジェクトの成功条件は、単に `.wasm` を出せることではない。TypeScript / JavaScript として意味のあるコードが、WASM 側で実行され、Node.js への依存が明示的に分離され、テストと性能の状態が継続的に測定されることが成功条件である。

最初の明確な成功ラインは、`docs/11-shared-definitions.md` の canonical milestones に従う。個別文書では milestone を再定義せず、必要に応じて `M0`、`M1` のように参照する。

M0 は実装前の土台であり、runtime ABI、capability manifest、test status schema を固定する段階である。M1 から M3 は縦切りと差分 fixture の足場作りであり、compile-time evaluator の使用を一時的に許す。M4 で JS/TS 意味論を WASM runtime 上へ移す。M5 から M6 では standalone WASI で動く最小実行系を広げ、stdin を WASI `fd_read` で扱う。M7 から M8 で差分実行と coverage 管理を運用可能にする。M9 以降は型情報を使った高速化と Node host が必要な API の明示的な分離を進める。

## First slice

```text
single file TS/JS
→ JS semantic IR
→ runtime ABI call
→ WASI wasm
→ iwasm execution
→ Node differential test
→ capability manifest
```
