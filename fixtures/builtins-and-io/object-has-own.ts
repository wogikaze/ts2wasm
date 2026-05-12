const obj = { a: 1 };
const inherited = Object.create(obj);
inherited.b = 2;

console.log(Object.hasOwn(obj, "a"));
console.log(Object.hasOwn(obj, "b"));
console.log(Object.hasOwn(inherited, "a"));
console.log(Object.hasOwn(inherited, "b"));
