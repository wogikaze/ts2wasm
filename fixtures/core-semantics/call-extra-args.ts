// Calling a function with more arguments than declared parameters
// Extra arguments are silently ignored in JavaScript.
// The ts2wasm TypeScript compiler *rejects* extra args at compile time
// when using direct calls with statically-known function signatures.
// This fixture tests the rejection boundary.

function sum(a, b) {
  return a + b;
}

// Direct call with exact args — works
console.log(sum(1, 2));
// Direct call with extra args — rejected by TypeScript arity checker
console.log(sum(1, 2));
