// ================================================================
// Bun victory benchmark tier 1 — numeric-factorial
// Iterative factorial computation with repeated calls.
// Tests loop arithmetic, variable mutation, and function call
// overhead in the interpreter.
// ================================================================
// Expected output: 1401602227200

function fact(n: number): number {
  let r: number = 1;
  for (let i = 2; i <= n; i++) {
    r *= i;
  }
  return r;
}

function Main(): void {
  // Sum of factorials from 10! to 15! — stays within the safe
  // integer range (2^53) to ensure reproducible results.
  let total: number = 0;
  for (let n = 10; n <= 15; n++) {
    total += fact(n);
  }
  console.log(String(total));
}
