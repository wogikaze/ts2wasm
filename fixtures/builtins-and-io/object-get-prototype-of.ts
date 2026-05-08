// Object.getPrototypeOf tests
let proto = { x: 42 };
let obj = Object.create(proto);
let objProto = Object.getPrototypeOf(obj);
console.log(objProto.x);

let plain = {};
let plainProto = Object.getPrototypeOf(plain);
console.log(plainProto === Object.prototype);

// null prototype
let nullObj = Object.create(null);
let nullProto = Object.getPrototypeOf(nullObj);
console.log(nullProto === null);
