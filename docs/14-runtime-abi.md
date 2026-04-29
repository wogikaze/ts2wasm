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

### BigInt value representation (accepted design)

BigInt は **heap object representation** を採用する。現行 `RawValue` の下位 3bit tag はすでに immediate / array / string / object で使い切っているため、BigInt 専用 immediate tag は追加しない。BigInt `RawValue` は `object` tag (`ptr | 0b111`) を使い、GC heap header の object kind で BigInt payload として判別する。

選択理由:

- BigInt は任意精度整数であり、small immediate だけでは ECMA-262 の値域を表せない
- `RawValue` の tag 空間を拡張すると既存 array/string/object wire encoding と backend helper 全体に波及する
- GC heap object に統一すると literal / arithmetic / boxed builtin boundary / future Wasm GC backend の差し替えを同じ論理 ABI で扱える

BigInt object payload は canonical little-endian limb sequence とする。

```text
BigInt payload:

  +0: i32 sign        ; -1 negative, 0 zero, 1 positive
  +4: i32 limb_count  ; canonical zero は sign=0, limb_count=0
  +8: u64 limbs[limb_count] little-endian magnitude limbs
```

Canonicalization rules:

- zero は必ず `sign=0, limb_count=0` に正規化する
- non-zero は `sign` を `-1` または `1` にし、最上位 limb の zero padding を持たない
- `-0n` は ECMA-262 と同じく `0n` と同一値に正規化する
- 文字列化は `n` suffix を付けない decimal string を返す

`RawValue` 判定は次の順で行う。

1. 下位 tag が immediate / array / string / object のどれかを判定する
2. object tag の場合だけ heap header kind を読む
3. heap kind が `bigint` のとき BigInt payload として扱う

BigInt object は GC mark 対象だが、payload は primitive limb array なので子参照を持たない。interned BigInt は当面作らず、literal lowering は runtime constructor で heap allocation する。

### BigInt runtime ABI boundary

BigInt を扱う runtime helper は論理 `jsval` を入出力に使う。現行 wasm wire では `RawValue i32` を返し、将来の `jsval i64` bridge では同じ論理 helper 名を維持する。

| Logical helper | Signature | First implementation owner | Notes |
|---|---|---|---|
| `make_bigint_literal` | `(ptr digits, len, radix, negative) -> jsval` | issue 256 | Source literal digits を runtime が canonical limb に変換する |
| `bigint_to_string` | `(jsval) -> jsval` | issue 259 | decimal string。`n` suffix は含めない |
| `bigint_to_boolean` | `(jsval) -> bool` | issue 256 | `0n` は false、それ以外は true |
| `bigint_strict_equal` | `(jsval, jsval) -> bool` | issue 258 | BigInt 同士は mathematical value 比較。Number とは常に false |
| `bigint_abstract_equal` | `(jsval, jsval) -> bool` | issue 258 | Number/String/Boolean との ECMA-262 coercion 境界を実装する |
| `bigint_compare` | `(op, jsval, jsval) -> jsval` | issue 258 | `< <= > >=`。成功時 bool `jsval`、例外時 pending exception |
| `bigint_add` / `sub` / `mul` / `div` / `rem` | `(jsval, jsval) -> jsval` | issue 257 | BigInt 同士のみ。Number 混在は TypeError |
| `bigint_unary_minus` | `(jsval) -> jsval` | issue 257 | `-0n` は `0n` |

IR は BigInt literal と BigInt operations を phase-specific に扱う。

- Parser/frontend: BigInt syntax classification only。invalid literal syntax は issue 244 の diagnostics を維持する
- Resolver/BuiltinResolver: BigInt literal node を runtime-capable expression として残し、未実装 operation は該当 implementation issue ID を含む source diagnostic にする
- Lowering: literal は `BigIntLiteral { raw, radix, negative }` 相当の semantic/lowered node へ落とし、backend が runtime constructor を選ぶ。BigInt operation は mixed Number/BigInt TypeError path を runtime helper に委譲する
- Backend/runtime link plan: BigInt helper は `RuntimeFn` catalog で deps/imports/capabilities/runtime strings を持つ。BigInt だけでは host import を要求しない

Unsupported boundary:

- literal runtime values until issue 256: `unsupported-bigint-runtime` / issue 256
- BigInt arithmetic and unary operators until issue 257: `unsupported-bigint-arithmetic` / issue 257
- BigInt equality, relational comparison, and coercion until issue 258: `unsupported-bigint-comparison` / issue 258
- BigInt builtin functions and string conversion until issue 259: `unsupported-bigint-builtin` / issue 259

Broad BigInt implementation must not be hidden inside parser, backend emitter, or generic number helpers. Each slice must update docs/current-state/tests with Node differential evidence for the exact operation class it enables.

## Planned Heap GC strategy (017a)

This section records the planned GC model for the current runtime. The implementation is not in this issue.

### Chosen strategy

**Stop-the-world mark-and-sweep** is selected as the baseline strategy.

Rationale:

- 長寿命の `closure`/`class`/`module` オブジェクトが増えるケースを安全側に扱える
- `arena` は明示的な生存区間が必要で、現行の型付けと実行モデルでは閉じ込めが困難
- `string`/`array`/`object` が同一ヒープを使う現状では、最初に `mark+list-based sweep` を導入するのが既存 runtime の変更面積を最小化できる

### Planned heap object header

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

### GC trigger points

`$alloc_heap` は以下のどちらかを満たすと GC を試行する:

- `alloc_bytes_since_last_gc >= 4096`
- `next_free >= memory.size * 0x80 / 100` （メモリ使用率が 80% を超える）

Pseudo flow:

1. markフェーズ: ルートとして `globals` / `runtime stacks` / `module cache` を走査
2. sweepフェーズ: 生存フラグがないブロックを `sweep_list` へ回収（空きリストへ）
3. 回収後に再試行、必要なら `memory.grow`、それでも足りなければ trap

### Safety and compatibility notes

- この設計は最初の実装では stop-the-world 全停止 GC とし、同時実行は対象外
- mark ビットは各 GC サイクルで反転 bit を使って O(1) リセットする方式を採用（全ヒープ走査の clear を回避）
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

## GC Strategy

### Choice: Mark-and-Sweep GC

初期実装ではシンプルな mark-and-sweep GC を採用する。

**理由:**
- 現在の bump allocator からの移行が容易
- Arena allocator は allocation pattern の大幅な変更が必要
- 短命プログラム (CLI tools) では GC 頻度が低く、パフォーマンス影響が限定的
- 将来的に generational GC への移行が可能

### Heap Object Header Design

すべての heap object は以下の header を持つ:

```text
[offset + 0 .. +4)   : i32 size (バイト数、header を含む)
[offset + 4 .. +8)   : i32 type_tag (object type と mark bit をエンコード)
[offset + 8 .. +N)   : type-specific payload
```

type_tag encoding:
- bit 0-7: object type (OBJECT, ARRAY, STRING, CLOSURE)
- bit 31: mark bit (GC mark phase で使用)

定数は `runtime/layout.rs` の `Layout` で定義:
- `OBJECT_HEADER_SIZE`: 8 (size + type_tag)
- `OBJECT_TYPE_MASK`: 0x7F
- `GC_MARK_BIT`: 0x80000000

### GC Trigger Points

GC は以下のタイミングで実行:

1. **Allocation threshold**: `$heap` が `HEAP_START + GC_THRESHOLD` を超えた場合
   - `GC_THRESHOLD` は初期値として 64KB
   - threshold は GC 後に動的に調整可能

2. **Explicit collection**: 将来的に `gc()` API を追加可能

### Mark Phase

Mark phase は以下の root set から開始:

1. Global variables (将来の実装)
2. Top-level local root table and active function call-frame roots
3. Runtime strings (interned strings は GC 対象外)

Function call-frame roots are stored in a fixed root-frame stack allocated once
from `_start` as part of the GC root table allocation. Function entry registers a
frame containing the previous-frame pointer, slot count, and mirrored local
slots; every function return unregisters that frame before returning the saved
result. This avoids allocating during function prologue, so heap parameters are
not exposed to collection before registration.

Mark algorithm:

```
mark(root):
  if root is heap object:
    if not marked:
      set mark bit
      for each reference in object:
        mark(reference)
```

### Sweep Phase

Sweep phase は heap を走査し、unmarked objects を回収:

```
sweep():
  ptr = HEAP_START
  while ptr < $heap:
    size = i32.load(ptr)
    type_tag = i32.load(ptr + 4)
    if not marked:
      free(ptr, size)
    else:
      clear mark bit
    ptr += size
```

### Implementation Notes

- 初期実装では stack locals の追跡は簡略化 (GC 時に stack frame を走査)
- Interned strings は GC 対象外 (static data segment)
- 将来的に write barrier を追加して generational GC へ移行可能
