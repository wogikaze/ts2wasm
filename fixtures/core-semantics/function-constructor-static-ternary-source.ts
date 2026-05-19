let truthy = Function(true ? "return 1" : "return 2");
let falsy = Function(false ? "return 1" : "return 2");
let fallback = Function("" ? "return 1" : 1 + 2);

console.log(truthy());
console.log(falsy());
console.log(fallback());
console.log(fallback.toString());
