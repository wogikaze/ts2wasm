let copy = { left: 0, ...{ a: 1, b: 2 }, b: 3, ...{ c: 4 } };

console.log(copy.left);
console.log(copy.a);
console.log(copy.b);
console.log(copy.c);
