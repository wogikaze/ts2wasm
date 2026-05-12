// Calling a function with fewer arguments than declared parameters
// ts2wasm TypeScript compiler rejects this at compile time.

function sum(a, b) {
  return a + b;
}

console.log(sum(5));
