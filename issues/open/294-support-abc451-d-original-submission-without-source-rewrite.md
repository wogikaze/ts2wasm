---
id: 294
title: "Support ABC451 D original submission without source rewrite"
type: feature
area: frontend/runtime
class: blocked
priority: P1
depends_on: [274]
blocks: []
created: 2026-04-29
updated: 2026-04-29
---

## Summary

Support the original AtCoder ABC451 D TypeScript/Bun submission as ordinary user code, without detecting that specific source and substituting a different implementation.

Problem: The ABC451 D original submission currently fails before wasm generation because several common TypeScript/JavaScript constructs and the broader number model are not implemented together.

## Current failure

Reproduction:

```sh
cat > /tmp/abc451-original-bun.ts <<'TS'
function search(before: string, powersOfTwoStr: string[]): string[] {
    const answers: string[] = [];
    if (before.length > 0) answers.push(before);
    const remainDigits = 9 - before.length;
    for (let i = 0; i < powersOfTwoStr.length; i++) {
        const after = powersOfTwoStr[i];
        if (after.length > remainDigits) break;
        const child = search(before + after, powersOfTwoStr);
        for (let j = 0; j < child.length; j++) {
            answers.push(child[j]);
        }
    }
    return answers;
};

function Main(inputText: string): void {
    const input: string[][] = inputText.trim().split("\n").map(row => row.split(" "));
    const N: number = +input[0][0];
    const powersOfTwo: number[] = [];
    for (let i = 0; 2 ** i <= 1e9; i++) {
        powersOfTwo.push(2 ** i);
    }
    const powersOfTwoStr: string[] = powersOfTwo.map(n => String(n));
    const allGoodIntStr = search("", powersOfTwoStr);
    const allGoodIntHasDup = allGoodIntStr.map(n => +n);
    const allGoodIntSet = new Set<number>();
    for (let i = 0; i < allGoodIntHasDup.length; i++) {
        allGoodIntSet.add(allGoodIntHasDup[i]);
    }
    const allGoodInt = [...allGoodIntSet];
    allGoodInt.sort((a, b) => a - b);
    console.log(allGoodInt[N - 1]);
    return;
}

export { };
// @ts-ignore
Main(await Bun.file("/dev/stdin").text());
TS

cargo run -q -- build /tmp/abc451-original-bun.ts -o /tmp/abc451-original-bun.wasm --host-deny
```

Current first failure:

```text
error: [UnsupportedSyntax] expected Semicolon, got Some(Ident("e9"))
```

After that first parser gap, the same source also requires:

- exponent numeric literals such as `1e9`
- top-level `await` or a Bun stdin compatibility lowering for `await Bun.file("/dev/stdin").text()`
- `Array.prototype.map` callback dispatch and result-array allocation
- unary plus numeric conversion (`+input[0][0]`, `+n`)
- `String(value)` conversion for dynamic numbers
- spread over `Set` iteration (`[...allGoodIntSet]`)
- `Array.prototype.sort((a, b) => a - b)`
- number semantics beyond the current tagged small-int range, because expected outputs include values such as `819264512`

## Desired final state

The original Bun submission builds through the normal frontend/resolver/lowering/backend pipeline, without source-text special casing, and runs under `iwasm` with Node/Bun-equivalent stdout for the official samples:

```text
10       -> 21
69       -> 328
1099898  -> 819264512
```

The generated manifest remains standalone under `--host-deny`, using only WASI stdin/stdout for this program.

## Scope

In scope:

- [ ] Add parser support for decimal exponent numeric literals, with correct diagnostics for invalid forms.
- [ ] Define and implement a portable stdin lowering policy for the Bun submission form or implement the needed top-level await/Bun facade subset.
- [ ] Implement `Array.prototype.map` for direct callbacks used by the fixture.
- [ ] Implement dynamic `String(value)` and unary plus conversion for the fixture's string/number paths.
- [ ] Implement Set iteration through array spread, or split a child issue that depends on issue 274 and covers iterable spread semantics.
- [ ] Implement `Array.prototype.sort` with comparator callback for number arrays.
- [ ] Extend the number representation or add a proven lowering path so values up to `1_000_000_000` and sample output `819264512` are represented correctly.

Out of scope:

- Source-text pattern matching for this AtCoder problem.
- Replacing the user program with a generated DP/table implementation.
- Full Bun API compatibility beyond the stdin form needed here, unless chosen as an explicit design decision.

## Affected paths

Expected:

- `crates/frontend/src/lexer.rs`
- `crates/frontend/src/parser/`
- `crates/ir/src/`
- `crates/backend-wasm/src/`
- `crates/runtime-abi/src/`
- `crates/cli/tests/`
- `fixtures/atcoder/`
- `docs/05-compatibility-and-semantics.md`
- `current-state.md`

Do not touch:

- problem-specific source rewrite hooks
- generated replacement implementations for a single contest task

## Acceptance criteria

- [ ] A fixture containing the original ABC451 D Bun submission builds with `cargo run -q -- build <fixture> -o <wasm> --host-deny`.
- [ ] The resulting wasm runs under `iwasm` and prints `21`, `328`, and `819264512` for the three official sample inputs.
- [ ] The manifest for the fixture has `standalone: true`, `node_host.required: false`, and no node host imports.
- [ ] Regression tests cover each newly supported language/runtime feature separately, not only the contest fixture.
- [ ] No code path detects this specific contest source text and substitutes another program.
- [ ] Docs/current-state describe the newly supported subsets and remaining limitations.

## Validation

Required commands:

```sh
cargo fmt --all --check
cargo nextest run
mise run check issues
```

Impacted commands:

```sh
cargo run -q -- build fixtures/atcoder/abc451-d-concat-power2.ts -o /tmp/abc451-d.wasm --emit-manifest /tmp/abc451-d.manifest.json --host-deny
printf '10\n' | iwasm /tmp/abc451-d.wasm
printf '69\n' | iwasm /tmp/abc451-d.wasm
printf '1099898\n' | iwasm /tmp/abc451-d.wasm
```

Not run:

- none

## Docs / current-state / issue sync

Final-state docs:

- [ ] not affected
- [ ] updated: `docs/05-compatibility-and-semantics.md`

Current state:

- [ ] not affected
- [ ] updated: `current-state.md` (repo root)

Follow-up issues:

- [ ] none
- [ ] created/updated: child issues for numeric model, Bun stdin/top-level await, Array.map, Set spread, Array.sort, and conversion semantics if this issue is split before implementation.

## Notes

This issue is intentionally blocked because the fixture crosses several existing feature boundaries. It should be split into implementation-ready child issues before execution unless the number model and iterable/callback semantics are already being handled in a broader milestone.

2026-04-29 progress:

- Added parser/lexer support for positive decimal exponent numeric literals in
  the current integer-backed number model, covering the ABC451 source fragment
  `1e9`.
- Negative decimal exponents such as `1e-3` remain source-spanned issue-294
  diagnostics because they require fractional number support.
- Decimal BigInt literals with exponents, such as `1e2n`, still report the
  existing issue-244 diagnostic and are not reclassified as number literals.
- The original ABC451 repro now advances past `1e9` and reaches the next
  unsupported boundary near the empty export / top-level await tail.
- This does not close issue 294; top-level `await`, Bun stdin, `Array.map`,
  unary plus, `String(value)`, Set spread, `Array.sort`, and broader number
  representation work remain.

## Completion evidence

Fill only when moving to `done/`.

Commits:

- `...`

Validation result:

```text
command:
result:
date:
```

Remaining risks:

- none
