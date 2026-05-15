let source = { a: 1, b: 2 };
let key = "c";
let object = { prefix: 0, ...source, [key]: 3 };

console.log(object.prefix);
console.log(object.a);
console.log(object.b);
console.log(object.c);
