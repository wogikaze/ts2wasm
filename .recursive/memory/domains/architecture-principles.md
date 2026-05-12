# Design Principles (from 2026-05-11 architecture review)

## Triple Separation Principle

This project needs three independent boundaries, not just layer separation:

1. **Phase boundary** — when to decide: Parser → NameResolver → BuiltinResolver → HIR → MIR → RuntimeLinkPlan → Backend
2. **Semantic domain boundary** — what semantics: array, object, class, module, builtin/host, async, string/regexp, number/bigint
3. **Capability boundary** — external capability required: WASI, filesystem, clock, random, Node shim

## Design Slogans

```
Parser は知らない。
Resolver は決めない。
BuiltinResolver は emit しない。
HIR は layout を知らない。
MIR は syntax を知らない。
RuntimeLinkPlan は WAT を知らない。
Backend は semantics を決めない。
CLI は compiler を知らない。
```

## Concrete Rules

```
名前文字列は NameResolver 以降に残さない。
runtime 関数名文字列は IR に残さない。
host import 文字列は backend に直書きしない。
source 起因 error は必ず span を持つ。
compiler bug は InvariantViolation にする。
backend は validate 済み IR 以外を受けない。
capability は RuntimeSpec → RuntimeLinkPlan → Manifest のみで決まる。
```

## Status: CURRENT (2026-05-11)
