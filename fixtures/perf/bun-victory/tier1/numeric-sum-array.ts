// ================================================================
// Bun victory benchmark tier 1 — numeric-sum-array
// For loop summing an array of numbers.
// Bun typically beats ts2wasm on tight numeric loops due to
// JIT compilation vs interpreter overhead.
// ================================================================
// Expected output: 4999950000

function Main(): void {
  const N: number = 100000;
  const arr: number[] = [];

  for (let i = 0; i < N; i++) {
    arr.push(i);
  }

  let sum: number = 0;
  for (let i = 0; i < arr.length; i++) {
    sum += arr[i];
  }

  console.log(String(sum));
}
