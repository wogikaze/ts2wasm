# UnresolvedName builtins burn-down — Updated Plan (2026-05-17)

## Current Status (test262-53469)

| Metric | Baseline | Current | Target |
|--------|----------|---------|--------|
| Build Pass | 6,448 | **14,542** | 53,469 |
| Unsupported (Skipped) | 27,930 | **28,236** | **0** |
| UnresolvedName | 8,471 | **4,191** | **0** |
| Runtime Error | 18,857 | **10,435** | 0 |
| UnsupportedBuiltin (explicit) | 0 | 2,427 | — |
| Oracle Skipped | 1,936 | 10,926 | — |
| Blocked | 220 | 218 | 0 |
| Total | 53,469 | 53,469 | 53,469 |

## 最終目標: Skipped (Unsupported) = 0

Skipped は全テストが何らかの結果 (build_pass / runtime_error / blocked) に分類される状態を指す。unsupported は「コンパイラが未対応のためスキップ」であり、未対応機能が残っている限り0にならない。よって「Skipped = 0」は全テストが build/runtime に到達することを意味する。

Skipped 0 は中長期的な目標であり、この計画では UnresolvedName 0 を短期的マイルストンとする。

## 残り Unsupported 28,236 の内訳

### UnsupportedSyntax: 13,662 (48%)
未対応の構文機能。主なもの:
- class fields / private fields / decorators
- 新しい構文 (do expressions, pipeline operator 等)
- 未対応の built-in API 呼び出し

→ 別 issue でトラック。この計画の対象外。

### SyntaxError: 5,951 (21%) ← 修正対象 (2026-05-17: can_pass_compile_negative 修正)
テストが期待通り SyntaxError を出すケース。negative test の正常系。
→ `negative: { phase: early, type: SyntaxError }` を持つテストは build_pass に再分類すべき。
→ `scripts/lib/test262_harness.py` の `can_pass_compile_negative` が `phase: early` を無視していた。
   修正: `"early"` を許容し、`ast-validator` phase も追加。これにより SyntaxError ケースの大部分が
   negative_pass として正しくカウントされる見込み。

### UnresolvedName: 4,191 (15%) ← 修正対象

**テストローカル変数 (~3,500):** 従来「本質的に unresolvable」としていたが、正しくは**名前解決器のスコープ追跡のバグ**。
公式 test262 ハーネスがコンパイルを通るようにするのが正しい方策。
例:
- `var codePoint = str.codePointAt(0)` → `var` 宣言が追跡できていない
- `testWithTypedArrayConstructors(function(TA) { new TA(3); })` → コールバック引数 `TA` がスコープ外
- クロージャ環境・関数スコープの境界で変数が見えなくなっている

→ 別 issue でスコープ解決の改善をトラック。stub でごまかさない。

**harness 関連 (~700):** `$DETACHBUFFER`, `testWithTypedArrayConstructors` 等、harness ファイル由来の名前。
preprocessor の real content inject で解決済みのはずだが、未解決のケースがある。調査中。

### UnsupportedBuiltin: 2,427 (9%) ← 明示 unsupported 済み
Temporal (2,042), Intl sub-APIs (~300), ShadowRealm (~50), Float16Array (~35)
→ UnresolvedName から UnsupportedBuiltin への再分類完了。

### 残り: ~2,000
ExpectedNegativeSyntax (965), UnresolvedFunction (516), UnsupportedModule (335), DuplicateLocal (96), NegativeCompileMismatch (39), DuplicateFunction (22), NegativeRuntimeUnverified (15), ArityMismatch (10), DuplicateParameter (5), UnsupportedEval (2)

→ 別 issue でトラック。

## 作業優先順位 (更新)

### Phase 1: Preprocessor harness stubs 拡充 (即効)
UnresolvedName 4,191 のうち harness 関連 ~700 を削る:
- `$DETACHBUFFER` → stub 追加
- `testWithBigIntTypedArrayConstructors` → stub 対象として扱う (第1引数関数名)
- `testWithTypedArrayConstructors` → 同上
- `testWithAtomicsFriendlyTypedArrayConstructors` → 同上
- `anyTypedArrayConstructors`, `nonClampedIntArrayConstructors` → 同上
- `assert` → 不足ケースを調査

### Phase 2: Iterator helpers runtime wiring
Iterator.from と $get_iterator / $iterator_next (host call) は完了。
残り: prototype helpers (map/filter/take/drop/toArray 等) の WAT 実装。

### Phase 3: 残 UnresolvedName 4,191 の精査
テストローカル変数 (codePoint, f, x, yield, instance 等) が UnresolvedName として正しいか確認。
- `codePoint` (452): test262 の `String.prototype.codePointAt` テストで使われる変数。テスト内で `var codePoint = ...` と宣言されるはず → 宣言前の参照があればバグ
- テスト変数は本質的に UnresolvedName として正しい

### Phase 4: 長期的なロードマップ
- UnsupportedSyntax 13,662 → 構文実装ごとに削減
- Runtime Error 10,435 → バグ修正ごとに削減
- Skipped 0 は 2027年目標

## 検証コマンド

```bash
cargo nextest run                          # 全テスト
mise run reference-coverage -- test262 --jsonl --sample 500 --jobs 4  # カバレッジ計測
mise run coverage-dashboard-data           # ダッシュボード更新
```
