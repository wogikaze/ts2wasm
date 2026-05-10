// W5.4: Freeze detection and Object.keys after freeze
// Note: property-level write/delete enforcement is not yet implemented

const obj = { a: 1, b: 2, c: 3 };
Object.freeze(obj);

console.log(Object.isFrozen(obj));  // true
console.log(obj.a);                 // 1 — value preserved

// Object.keys still works after freeze
let keys = Object.keys(obj);
console.log(keys.length);           // 3
