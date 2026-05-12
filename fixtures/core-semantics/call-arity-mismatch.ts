// Argument count edge cases at the TypeScript/JavaScript boundary
// ts2wasm enforces TypeScript arity rules at compile time.
// These calls have mismatched argument counts and should be rejected.

function sum(a, b) {
  return a + b;
}

function identity(x) {
  return x;
}

function noParams() {
  return 42;
}

function concat(a, b, c) {
  return a + b + c;
}

// Call with exact args — should compile
console.log(sum(1, 2));
console.log(identity(7));
console.log(noParams());
console.log(concat("x", "y", "z"));
