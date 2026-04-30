let source = { a: 1, b: 2 };
source.a = 4;
source.c = 5;

let copy = { z: 0, ...source, b: 3 };

console.log(copy.z);
console.log(copy.a);
console.log(copy.b);
console.log(copy.c);
