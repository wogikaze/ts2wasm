// Expected diagnostic: literal Function constructor sources use the
// FormalParameters parse goal. Parameter text that escapes the synthetic wrapper
// must be rejected before AOT expansion can produce a generated function.
let f = Function("a) { return 1; } function injected(", "return 2");
console.log(f());
