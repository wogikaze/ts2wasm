// ================================================================
// Bun victory benchmark tier 1 — numeric-sum-raw
// For loop summing numbers directly (no array).
// Minimal loop overhead — pure arithmetic pipeline.
// Bun's JIT can hoist invariant bounds and eliminate
// array bounds checks that ts2wasm's interpreter must pay.
// ================================================================
// Expected output: 4999950000

function Main(): void {
  const N: number = 100000;
  let sum: number = 0;

  for (let i = 0; i < N; i++) {
    sum += i;
  }

  console.log(String(sum));
}
