// Inline property access on object literal
let val = { a: 100 }.a;
console.log(val);

// Inline nested access
let nested = { a: { b: 200 } }.a.b;
console.log(nested);

// Object literal in expression
let sum = { a: 5, b: 10 }.a + { c: 20, d: 30 }.c;
console.log(sum);

// Access missing property on literal
let undef = {}.x;
console.log(undef);

// Chained inline access
let result = { x: { y: { z: 42 } } }.x.y.z;
console.log(result);
