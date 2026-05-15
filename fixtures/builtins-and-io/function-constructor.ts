// Function constructor (issue I-20260513-B49ZZE)
// Runtime code evaluation is intentionally unsupported in ts2wasm.
// This fixture should produce an UnsupportedEval diagnostic.
var fn = new Function("a", "b", "return a + b");
console.log(fn(1, 2));
