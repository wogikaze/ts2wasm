const proto = { x: 10 };
const obj = Object.create(proto);
console.log("create with proto:", obj.x);

// Object.create with null prototype
const objNull = Object.create(null);
objNull.y = 20;
console.log("create null proto obj.y:", objNull.y);

// Object.keys on a simple object
const keysObj = { a: 1, b: 2 };
console.log("keys:", Object.keys(keysObj).length);
