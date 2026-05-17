// Function constructor (issue I-20260513-B49ZZE)
// Literal-only Function constructor arguments are expanded at compile time.
var fn = new Function("a", "b", "return a + b");
console.log(fn(1, 2));
