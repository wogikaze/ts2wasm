# Runtime ABI

このドキュメントは ts2wasm の runtime ABI を定める。
`RawValue` の tagged representation、heap レイアウト、`RuntimeFn` カタログ、host import ABI を定義する。

## Tagged i32 value representation（small-int subset）

### RawValue (i32 tagged encoding)

現行パイプラインでは JavaScript の値を wasm モジュール内で `i32` の tagged encoding として表現する。
これは意図的な **small-int subset** であり、JS 全体の `number` セマンティクスではない。

> **論理 ABI との関係**: `docs/04-compiler-architecture-and-runtime.md` の論理 ABI では `jsval` を `i64` として定義し、`crates/shared/src/abi.rs` の `AbiType::JsVal` もそれに従う。`i32` RawValue は wasm 本体の wire 表現であり、論理 `i64` との変換は明示的な bridge のみで行う。backend は wire と論理表現を暗黙に混在させてはならない。

```text
i32 tagged value (RawValue):

  undefined: 0b000 = 0
  null:      0b001 = 1
  false:     0b010 = 2
  true:      0b011 = 3
  number:    (n << 3) | 0b100   — n は i32 の範囲内の整数のみ
  array:     ptr | 0b101        — ptr はヒープ上の Array object のアドレス (8-byte aligned)
  string:    ptr | 0b110        — ptr はヒープ上の String object のアドレス (8-byte aligned)
  object:    ptr | 0b111        — ptr はヒープ上の Object のアドレス (8-byte aligned)

TAG_MASK:  0b111
HEAP_MASK: !0b111 (= -8 in two's complement)
```

定数は `runtime/value.rs` の `ValueTag` で定義する。
backend が直接数値を埋め込むことは禁止。

## Memory Layout

```text
linear memory (4 MiB):

  [0 .. 8)                          — 予約
  [8 .. 16)                         — fd_write iovec (IOVEC_PTR=8, IOVEC_LEN=12)
  [16 .. 28)                        — stdin fd_read iovec + nread slot
                                       (STDIN_IOVEC_OFFSET=16, STDIN_IOVEC_PTR=16,
                                        STDIN_IOVEC_LEN=20, STDIN_NREAD_OFFSET=24)
  [28 .. DATA_START)                — 予約
  [DATA_START .. SCRATCH_OFFSET)    — static data segment (interned strings)
  [SCRATCH_OFFSET .. SCRATCH_OFFSET+SCRATCH_SIZE) — scratch buffer ($value_to_string_into 用)
  [STDIN_BUFFER_OFFSET .. STDIN_BUFFER_OFFSET+STDIN_BUFFER_SIZE) — stdin read staging buffer
  [HEAP_START .. )                  — heap ($heap global, bump allocator)
```

定数は `runtime/layout.rs` の `Layout` で定義する。

| 定数 | 値 | 用途 |
|---|---|---|
| `DATA_START` | 256 | interned string data の開始オフセット |
| `ALIGN` | 8 | data segment / heap の alignment |
| `SCRATCH_OFFSET` | 1500 | 一時バッファの開始オフセット |
| `SCRATCH_SIZE` | 256 | 一時バッファのサイズ |
| `HEAP_START` | 2048 | heap bump allocator の開始アドレス |
| `IOVEC_PTR` | 8 | fd_write iovec の ptr フィールドオフセット |
| `IOVEC_LEN` | 12 | fd_write iovec の len フィールドオフセット |
| `STDIN_IOVEC_OFFSET` | 16 | stdin fd_read iovec 構造体のベースオフセット |
| `STDIN_IOVEC_PTR` | 16 | stdin fd_read iovec の buf ptr フィールドオフセット |
| `STDIN_IOVEC_LEN` | 20 | stdin fd_read iovec の buf_len フィールドオフセット |
| `STDIN_NREAD_OFFSET` | 24 | fd_read が書き込む nread 値のオフセット |
| `STDIN_BUFFER_OFFSET` | 1792 | stdin read staging buffer の開始オフセット |
| `STDIN_BUFFER_SIZE` | 256 | stdin read staging buffer のサイズ |
| `STDIN_READ_LIMIT` | 65536 | 1 回の readFileSync(0) で読める最大バイト数 (64 KiB) |

### Heap String Object

interned string / runtime string はヒープ上に以下の形式で配置する。

```text
[offset + 0 .. +4)   : i32 length (バイト数)
[offset + 4 .. +4+N) : UTF-8 bytes

RawValue = ptr | 0b110  (ptr は 8-byte aligned)
```

ptr を取り出すには: `ptr = raw_value & HEAP_MASK`
length を読むには: `len = i32.load(ptr)`
文字列バイトは: `ptr + 4` から `len` バイト

## Heap GC strategy (017a, 017b)

This section records the GC model selected in 017a and what is currently implemented by 017b.

### Chosen strategy

**Stop-the-world mark-and-sweep** is the selected baseline strategy.

Rationale:

- 長寿命の `closure`/`class`/`module` オブジェクトが増えるケースを安全側に扱える
- `arena` は明示的な生存区間が必要で、現行の型付けと実行モデルでは閉じ込めが困難
- `string`/`array`/`object` が同一ヒープを使う現状では、最初に `mark+list-based sweep` を導入するのが既存 runtime の変更面積を最小化できる

### Heap object header (implemented in 017b)

GC enabled allocation uses a **separate runtime header** before each heap block.
ユーザから観測される `RawValue` ポインタは header より `GC_HEADER_SIZE` 分だけ進んだ本体先頭を指す。

```text
obj_ptr + -16: i32 flags_and_type    ; mark bit + type bits
obj_ptr + -12: i32 body_size_bytes   ; this object's payload size in bytes (aligned)
obj_ptr + -8 : i32 sweep_next        ; freelist / sweep list linkage
obj_ptr + -4 : i32 gen_or_reserved   ; optional next-generation field (future)
obj_ptr      : payload
```

Flag layout:

- bit0: `mark` (1 = live in current mark cycle)
- bit1: `finalizable` (reserved)
- bits2-4: heap kind (`001` string, `010` array, `011` object)
- bits5-31: reserved

### Heap payload layout (planned)

The existing logical payload shape is kept; header is additive.

- `string`: `[len:i32, bytes... ]`
- `array`:  `[len:i32, elem0, elem1, ...]` (`i32` raw values)
- `object`: `[property_count:i32, prototype_ptr:i32, (key:value)×N]`

`object` は既存仕様に合わせて `prototype_ptr` を保持し、`[[Prototype]]` 走査は将来の markフェーズと連携させる。

### GC trigger points (partially implemented in 017b)

`$alloc_heap` は以下のどちらかを満たすと GC を試行する:

- `alloc_bytes_since_last_gc >= 4096`
- `next_free >= memory.size * 0x80 / 100` （メモリ使用率が 80% を超える）

Current implementation in 017b:

1. `$alloc_heap` tracks a per-collection budget in `$alloc_bytes_since_last_gc`.
2. It triggers `$gc_collect` when either allocation budget or usage ratio threshold is exceeded.
3. `$gc_collect` currently resets allocation accounting and is a placeholder; mark-and-sweep body is pending (see 017b follow-up scope).

### Safety and compatibility notes

- 本実装では mark-and-sweep の本文は未実装。`$gc_collect` は暫定的に budget reset のみを実行。
- 実装完了後は stop-the-world、single-thread model を前提として mark/sweep を追加する。
- 文字列 primitive の一時 `scratch` は現在どおり GC 対象外

## RuntimeFn Catalog

runtime 関数は `RuntimeFn` カタログとして管理する（catalog 化が完了すれば linker が単一導線になる）。
現状は巨大な WAT template として `runtime_builder.rs` に存在するが、以下の形へ移行する。

```rust
pub struct RuntimeFn {
    pub name: RuntimeSymbol,
    pub deps: &'static [RuntimeSymbol],
    pub imports: &'static [HostImport],
    pub capabilities: &'static [Capability],
    pub emit: fn(&mut ModuleBuilder),
}
```

### Tree-shaking 方針

`console.log` を使っていない program には `$log`, `$write`, `fd_write` を含めない。
`+` 演算子を使っていない program には `$add`, `$concat` を含めない。
これは `RuntimeFn.deps` と `RuntimeFn.capabilities` を静的に解析することで実現する。

## Host Import ABI

host import は capability manifest から生成する。
backend が直接 import 文字列を持つことは禁止（`RuntimeLinkPlan` 由来に限定する）。

### API 分類

| 分類 | 説明 | 例 |
|---|---|---|
| Wasm-native | runtime 内で完結する | 算術演算、string concat、===  |
| WASI-backed | fd_read / fd_write / clock_time_get などで実装 | console.log, Date.now |
| Host-backed | Node.js host が必要 | process.argv, fs.readFileSync |
| Unsupported | compile error。必要なら fallback plan を表示 | crypto.createHash |

この分類は `docs/03-api-and-host-capability.md` と `docs/11-shared-definitions.md` に正式定義する。
backend が API 分類を決めることは禁止。必ず capability manifest / semantic pass を通す。

## Bump Allocator

`$heap` global は `HEAP_START` 初期値を持つ。
string alloc 時は以下の手順で行う。

```text
1. align_to($heap, ALIGN) → base
2. i32.store(base, len)
3. $copy(src, base + 4, len)
4. $heap = align_to(base + 4 + len, ALIGN)
5. return base | STRING_TAG
```

### OOM Handling

`$alloc_heap` は `memory.size` を使用して利用可能なメモリをチェックする。
割り当てが現在のメモリサイズを超える場合、`unreachable` で trap する。
これにより、大きな割り当てによる未定義動作やメモリ破損を防ぐ。
