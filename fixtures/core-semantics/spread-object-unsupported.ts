let source = { a: 1 };
source.a = 2;
let copy = { b: 2, ...source };
console.log(copy.a);
