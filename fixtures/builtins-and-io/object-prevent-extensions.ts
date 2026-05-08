const obj = { a: 1, b: 2 };
Object.preventExtensions(obj);
console.log(obj.a);
console.log(obj.b);
