// ================================================================
// Bun victory benchmark tier 1 — numeric-fibonacci
// Recursive Fibonacci computation.
// Heavy on function call overhead and recursion depth.
// Bun's JIT can inline and optimize recursive calls in ways
// ts2wasm's interpreter cannot match.
// ================================================================
// Expected output: 1134903170

function fib(n: number): number {
  if (n <= 1) return n;
  return fib(n - 1) + fib(n - 2);
}

function Main(): void {
  const result: number = fib(45);
  console.log(String(result));
}
