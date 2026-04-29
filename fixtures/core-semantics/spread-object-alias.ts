let base = { a: 1, b: 2 };
let values = base;
let copy = { z: 0, ...values, b: 3 };

console.log(copy.z);
console.log(copy.a);
console.log(copy.b);
