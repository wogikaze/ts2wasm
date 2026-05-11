# Coverage Runner Completeness: build_pass が semantic_pass を欠く条件

この文書は `reference-coverage.py` の計測パイプラインにおいて、
`build_pass=True` のアイテムが `semantic_pass` に到達しない条件を網羅的に分析し、
修正方針を定義する。

## 1. 問題

coverage dashboard で以下の状態が観測される。

```text
build_pass=1814
semantic_pass=0
mismatch=0
runtime_error=0
blocked=0
fail=0
```

`semantic_enabled=1`（Node, iwasm 共に利用可能）であるにもかかわらず、
semantic outcome（pass / mismatch / runtime_error / blocked）がすべて 0 である。

これは「実行比較して全件不一致」ではない。**semantic 比較に入っていない
build-only pass が 1814 件存在する**ことを示す。

## 2. 計測パイプラインの 3 つのコードパス

`reference-coverage.py` には以下の 3 つのコードパスが存在する。

### 2.1. サーバーバッチモード

```mermaid
flowchart LR
    A[pre-process<br/>_process_one_file] --> B[server batch build]
    B --> C{status == "ok"?}
    C -->|yes| D[_classify_build_response<br/>build_pass=True<br/>semantic deferred]
    C -->|no| E{negative test?}
    E -->|yes + verified| F[_mark_verified_negative_compile_pass<br/>build_pass+semantic_pass]
    E -->|no| G[unsupported/blocked/fail]
    D --> H{semantic_enabled?}
    H -->|yes| I[_complete_pair<br/>_complete_semantic_for_build_item]
    I --> J[semantic_pass/mismatch/error/blocked]
```

### 2.2. レガシーサブプロセスモード（`use_server=False`）

`_process_one_file` → `_process_one_file_inner`:

```
build 成功 + semantic_enabled → _run_semantic_check → semantic outcome
build 成功 + !semantic_enabled → build_pass のみ
build 失敗 → diag_code に応じて unsupported/blocked/fail
```

### 2.3. サブプロセスフォールバック（サーバー障害時）

`_parallel_subprocess_batch` → `_run_build_item_in_subprocess`:

```
build 成功 + semantic_enabled → _run_semantic_check → semantic outcome
build 成功 + !semantic_enabled → build_pass のみ
build 失敗 → diag_code に応じて unsupported/blocked/fail
```

## 3. 各パスにおけるカバレッジギャップ

### 3.1. サーバーバッチ: deferred semantic が到達しない

サーバーモードでは `_classify_build_response` が `server_mode_batch=True` で呼ばれ、
semantic 実行を defer する。

```python
# scripts/run/reference-coverage.py:2127-2138
def _classify_build_response(build_resp, item, semantic_enabled, tmp_dir, server_mode_batch=False):
    if build_resp["status"] == "ok":
        rm["build_pass"] = True
        if semantic_enabled and not server_mode_batch:
            _complete_semantic_for_build_item(item, rm, tmp_dir)
```

その後 `_complete_pair` で semantic を補充する。

```python
# line 2610-2623
if semantic_enabled:
    def _complete_pair(pair):
        item, result = pair
        if result["build_pass"] and not result["semantic_pass"]:
            result = _complete_semantic_for_build_item(item, result, tmp_dir)
```

`_complete_semantic_for_build_item` では、サーバーが wasm を生成した場合は
`item["wasm_path"]` を使い、無い場合は standalone `ts2wasm build` にフォールバックする。

```python
# line 2188-2219
server_wasm_path = item.get("wasm_path")
if server_wasm_path:
    _run_semantic_check(...)
    return result_metrics

build_result = subprocess.run(["ts2wasm build", ...])
if build_result.returncode == 0:
    _run_semantic_check(...)
else:
    result_metrics["blocked"] = True
    result_metrics["diag_code"] = "SemanticWasmEmitFailed"
```

**ギャップ**: 以下の条件が重なると `build_pass=True`, `semantic outcome なし`
のアイテムが生まれる。

1. `server_emit_wasm=false`（またはサーバーが wasm を返さない設定）
   → `item["wasm_path"]` がセットされない
2. fallback build が失敗する（`returncode != 0`）
   → `blocked=True` になるので、このケースだけは outcome が生まれる

したがって、現状のコードでは fallback 失敗時は blocked に分類されるため、
このパスでは「outcome なし」は発生しない。ただし、fallback build のコスト
（サーバーと同じ build を二重実行）は無視できない。

### 3.2. verified negative compile の semantic_pass 昇格

`_classify_build_response` の非-ok ブランチでは、
negative test（SyntaxError expectation）を verified compile pass として扱う。

```python
# line 2146-2152
metadata = item.get("metadata")
if metadata is not None and metadata.expects_negative:
    t262r = _ensure_test262_runner()
    if t262r.can_pass_compile_negative(metadata, diag_code, build_resp.get("phase", "")):
        _mark_verified_negative_compile_pass(rm, semantic_enabled)
        if detail_output:
            rm["detail_line"] = f"{detail_path}: build_pass: verified negative parse/SyntaxError"
```

`_mark_verified_negative_compile_pass` は `semantic_enabled=True` の場合に
`semantic_pass=True` も同時にセットする。

```python
# line 518-522
def _mark_verified_negative_compile_pass(metrics, semantic_enabled):
    metrics["build_pass"] = True
    if semantic_enabled:
        metrics["semantic_pass"] = True
```

**現在の状態**: サーバーモードでも `_classify_build_response` には
正しい `semantic_enabled` が渡されている（line 2605-2606）ため、
verified negative compile は正しく `semantic_pass` に昇格している。

```python
# line 2605-2606
result = _classify_build_response(
    build_response, item, semantic_enabled, tmp_dir, server_mode_batch=True
)
```

### 3.3. レガシーモード: semantic_enabled が False の場合

```python
# line 2418-2425
if build_result.returncode == 0:
    result_metrics["build_pass"] = True
    if semantic_enabled:
        _run_semantic_check(file_path, source_code, metadata, thread_tmp, out_wasm, result_metrics)
    if detail_output:
        result_metrics["detail_line"] = f"{detail_path}: build_pass"
    return result_metrics
```

`semantic_enabled=False` の場合は何もしない。Node / iwasm がない環境では
正しい動作。

**注意**: レガシーモードでは negative compile の verified チェックがない。
`_process_one_file_inner` は `_classify_build_response` を経由せず、
直接 diag_code を unsupported に分類する（line 2434-2458 に対応する
negative compile ハンドリングがない）。

## 4. build_pass の内訳分析

`semantic_pass=0, mismatch=0, runtime_error=0, blocked=0` が同時に発生する
唯一の条件は **すべての build_pass アイテムが semantic 実行に入らなかった**
ことである。以下のいずれかが該当する。

### 4.1. semantic_enabled が実質的に効いていない

```text
semantic_enabled=1        ← 正しく表示
build_pass=1814           ← 正しくカウント
semantic_pass=0           ← 0
```

`semantic_enabled=1` は単に「Node と iwasm が見つかった」ことを示す。
`_complete_pair` に到達する前に処理が終了した可能性がある。

### 4.2. 集計パスの見落とし

`_accumulate_case_result` はサーバーバッチの結果を以下のように集計する。

```python
# line 2244-2256
if result["build_pass"]:
    build_pass_count += 1
    if result["semantic_pass"]:
        semantic_pass_count += 1
    elif result["mismatch"]:
        mismatch_count += 1
    elif result["runtime_error"]:
        runtime_error_count += 1
    elif result["blocked"]:
        blocked_count += 1
    if result["detail_line"]:
        file_details.append(result["detail_line"])
    return
```

`build_pass=True` で `semantic_pass/mismatch/runtime_error/blocked` の
すべてが `False` の場合、単に `build_pass` にカウントされるのみで、
どの semantic outcome にも分類されない。

**この状態は「semantic 比較に進めなかった build_pass」として、
明示的に分類すべきである**（例: `build_only` カウンター）。

## 5. 修正方針

### 5.1. 集計時の build_only カウンター追加

`_accumulate_case_result` で、`build_pass` かつ semantic outcome がない
ケースを明示的にカウントする。

```python
# 追加カウンター
build_only_count = 0

if result["build_pass"]:
    build_pass_count += 1
    if result["semantic_pass"]:
        semantic_pass_count += 1
    elif result["mismatch"]:
        mismatch_count += 1
    elif result["runtime_error"]:
        runtime_error_count += 1
    elif result["blocked"]:
        blocked_count += 1
    else:
        # build_pass だが semantic outcome なし
        build_only_count += 1
        result["diag_code"] = "BuildOnly"
    ...
```

これにより以下の区別が可能になる。

```text
build_pass=1814
  ├─ semantic_pass=1203
  ├─ mismatch=45
  ├─ runtime_error=12
  ├─ blocked=340
  └─ build_only=214   ← 新規。semantic 実行に到達しなかった件数
```

### 5.2. legacy モードでも negative compile を verified 判定する

`_process_one_file_inner` の非-ok ブランチに、`_classify_build_response` と
同じ negative compile verified チェックを追加する。

```python
# _process_one_file_inner, line 2434 以降
if is_test262 and metadata is not None and metadata.expects_negative:
    if t262.can_pass_compile_negative(metadata, diag_code, diag_phase or ""):
        _mark_verified_negative_compile_pass(result_metrics, semantic_enabled)
        if detail_output:
            result_metrics["detail_line"] = f"{detail_path}: build_pass: verified negative parse/SyntaxError"
        return result_metrics
    else:
        result_metrics["unsupported"] = True
        result_metrics["diag_code"] = "NegativeCompileUnverified"
```

### 5.3. サーバーバッチの二重 build 排除

`_complete_semantic_for_build_item` の fallback build は、サーバーが
すでに同じ build を完了している。サーバーの wasm 生成がオンになっていない
場合、完全に無駄な build が走る。

対応方針（選択）:

- **A**: サーバーで常に `emit: "wasm"` を使う。`TS2WASM_SERVER_EMIT_WASM` の
  デフォルトを `"1"` のまま運用でカバーする。
- **B**: fallback build の前にサーバーの結果を使い回す方策を入れる（例:
  一時ファイルにサーバーの compile artifact を保持させる）。

### 5.4. 出力フォーマット拡張

標準出力に `build_only` を追加する。

```text
build_pass=1814
semantic_pass=1203
mismatch=45
runtime_error=12
blocked=340
build_only=214       ← 追加
```

これにより、集計の正確性を一目で確認できるようになる。

## 6. 検証方法

### 6.1. build_only が 0 であることの確認

```bash
mise run reference-coverage -- test262 --limit 500 --jobs 8
# → build_only=0 を確認
```

### 6.2. server mode と legacy mode の一致確認

```bash
# server mode
mise run reference-coverage -- test262 --limit 200 --jobs 4

# legacy mode
mise run reference-coverage -- test262 --limit 200 --jobs 4 --no-server

# → build_pass, semantic_pass が一致することを確認
```

### 6.3. verified negative compile の追跡

```bash
# verified negative が semantic_pass に入ることを確認
mise run reference-coverage -- test262 --limit 2000 --detail 2>&1 | grep 'verified negative'
# → すべての verified negative が line に 'build_pass: verified negative' を含む
```

## 7. 参照

- `scripts/run/reference-coverage.py`: 計測スクリプト
- `docs/06-testing-and-coverage.md`: テスト分類・coverage 方針
- `docs/15-coverage-matrix.md`: coverage 運用・gate 基準
- `docs/17-jsonl-test-record-schema.md`: JSONL 出力スキーマ
