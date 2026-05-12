// Calling a function with more arguments than declared parameters
// ts2wasm TypeScript compiler rejects this at compile time.

function sum(a, b) {
  return a + b;
}

console.log(sum(1, 2, 3));
