// Object.seal basic test
const obj = { x: 1, y: 2 };
Object.seal(obj);
console.log(Object.isSealed(obj));
console.log(obj.x);
