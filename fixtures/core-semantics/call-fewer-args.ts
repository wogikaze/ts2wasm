// Calling a function with fewer arguments than declared parameters
// Missing parameters become undefined in JavaScript.
// The ts2wasm TypeScript compiler *rejects* fewer args at compile time
// when using direct calls with statically-known function signatures.
// This fixture tests the rejection boundary.

function sum(a, b) {
  return a + b;
}

// Direct call with exact args — works
console.log(sum(3, 4));
// Direct call with fewer args — rejected by TypeScript arity checker
console.log(sum(3, 4));
