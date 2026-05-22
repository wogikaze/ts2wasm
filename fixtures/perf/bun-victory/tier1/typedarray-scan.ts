// ================================================================
// Bun victory benchmark tier 1 — typedarray-scan
// TypedArray (Float64Array) element iteration with arithmetic.
// Bun's JIT can use SIMD or at least reduce bounds-check
// overhead for TypedArrays. ts2wasm's runtime dispatches
// through the host shim for every element access.
// ================================================================
// Expected output: 4999950000

function Main(): void {
  const N: number = 100000;
  const arr: Float64Array = new Float64Array(N);

  for (let i = 0; i < N; i++) {
    arr[i] = i;
  }

  let sum: number = 0;
  for (let i = 0; i < N; i++) {
    sum += arr[i];
  }

  console.log(String(sum));
}
