# WASI Features Reference

この文書は WASI (WebAssembly System Interface) の機能について、本プロジェクトでの対応方針と実装状況をまとめる。WASI 仕様は [WASI GitHub](https://github.com/WebAssembly/WASI) を正とする。

## 仕様リファレンス

| 仕様 | URL | 用途 |
|---|---|---|
| WASI Main | <https://github.com/WebAssembly/WASI> | WASI メインリポジトリ |
| WASI Preview 1 | <https://github.com/WebAssembly/WASI/blob/main/docs/Preview1.md> | Preview 1 仕様 |
| WASI Preview 2 | <https://github.com/WebAssembly/WASI/blob/main/docs/Preview2.md> | Preview 2 仕様 |
| WASI libc | <https://github.com/WebAssembly/wasi-libc> | WASI libc 実装 |
| WAMR WASI Support | <https://github.com/bytecodealliance/wasm-micro-runtime> | WAMR の WASI 実装 |

## WASI Preview 1 (標準)

| 機能 | 対応方針 | 実装状況 |
|---|---|---|
| `args_get` / `args_sizes_get` | コマンドライン引数 | 実装済み (WAMR) |
| `environ_get` / `environ_sizes_get` | 環境変数 | 実装済み (WAMR) |
| `clock_res_get` / `clock_time_get` | クロック | 未実装 |
| `fd_advise` | ファイルアクセスアドバイス | 未実装 |
| `fd_allocate` | ファイル領域確保 | 未実装 |
| `fd_close` | ファイル記述子クローズ | 未実装 |
| `fd_datasync` | データ同期 | 未実装 |
| `fd_fdstat_get` / `fd_fdstat_set_flags` | ファイル記述子状態 | 未実装 |
| `fd_filestat_get` / `fd_filestat_set_size` | ファイル状態 | 未実装 |
| `fd_pread` / `fd_pwrite` | ファイル読み書き | 未実装 |
| `fd_prestat_get` / `fd_prestat_dir_name` | preopen 情報 | 未実装 |
| `fd_read` / `fd_write` | ファイル読み書き (fd 0/1/2) | 実装済み (stdin/stdout/stderr) |
| `fd_readdir` | ディレクトリ読み込み | 未実装 |
| `fd_renumber` | ファイル記述子番号変更 | 未実装 |
| `fd_seek` / `fd_tell` | ファイルシーク | 未実装 |
| `fd_sync` | ファイル同期 | 未実装 |
| `path_create_directory` | ディレクトリ作成 | 未実装 |
| `path_filestat_get` / `path_filestat_set_size` | パス経由ファイル状態 | 未実装 |
| `path_link` | ハードリンク作成 | 未実装 |
| `path_open` | ファイルオープン | 未実装 |
| `path_readlink` | シンボリックリンク読み込み | 未実装 |
| `path_remove_directory` | ディレクトリ削除 | 未実装 |
| `path_rename` | ファイル/ディレクトリ名前変更 | 未実装 |
| `path_symlink` | シンボリックリンク作成 | 未実装 |
| `path_unlink_file` | ファイル削除 | 未実装 |
| `poll_oneoff` | イベント待機 | 未実装 |
| `proc_exit` | プロセス終了 | 未実装 |
| `proc_raise` | シグナル送信 | 未実装 |
| `random_get` | 乱数 | 未実装 |
| `sched_yield` | スケジューラ譲渡 | 未実装 |
| `sock_accept` | ソケット接続受付 | 実装済み (WAMR socket API) |
| `sock_recv` / `sock_send` | ソケット送受信 | 実装済み (WAMR socket API) |
| `sock_shutdown` | ソケットシャットダウン | 実装済み (WAMR socket API) |

## WASI Preview 2 (新世代)

| 機能 | 対応方針 | 実装状況 |
|---|---|---|
| Component Model 統合 | 型付き host interface | 将来対応 |
| WIT インターフェース | インターフェース定義 | 将来対応 |
| jco WASI Preview 2/3 shim | JS/TS 統合 | 将来対応 |
| 改良されたエラー処理 | エラー型システム | 将来対応 |
| 非同期 I/O | async I/O モデル | 将来対応 |

## WAMR 固有 WASI 拡張

| 機能 | 対応方針 | 実装状況 |
|---|---|---|
| libc-wasi library (~21.4K) | WASI libc | 実装済み (WAMR) |
| wasi-threads | POSIX スレッド | 実装済み (WAMR) |
| Berkeley/Posix Socket | ソケット API | 実装済み (WAMR) |
| multi-thread | マルチスレッド | 実装済み (WAMR) |
| AOT / JIT | コンパイルと実行 | 実装済み (WAMR) |

## Capability Mapping

| WASI 機能 | Capability | standalone |
|---|---|---:|
| `fd_read` (fd 0) | `wasi.stdin` | yes |
| `fd_write` (fd 1) | `wasi.stdout` | yes |
| `fd_write` (fd 2) | `wasi.stderr` | yes |
| `args_get` / `args_sizes_get` | `wasi.args` | yes |
| `environ_get` / `environ_sizes_get` | `wasi.env` | yes |
| `path_open` (read) | `wasi.filesystem.read` | 条件付き |
| `path_open` (write) | `wasi.filesystem.write` | 条件付き |
| `random_get` | `wasi.random` | yes |
| `sock_*` | `wasi.network` | yes (WAMR) |
| wasi-threads | `wasi.threads` | yes (WAMR) |

## 実装方針の原則

1. **WASI Preview 1 優先**: 初期は WASI Preview 1 に対応
2. **WAMR 活用**: WAMR の WASI 実装を活用 (libc-wasi, socket, threads)
3. **最小依存**: 必要な WASI 機能だけを manifest に記録
4. **Preview 2 準備**: 将来的な Preview 2 対応を見据えた設計
5. **Capability ベース**: WASI 機能を capability として管理
