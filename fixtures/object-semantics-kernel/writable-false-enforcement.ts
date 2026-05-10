// W5.1: Freeze detection via isFrozen
// Note: property-level writable enforcement is not yet implemented

const obj = { x: 10 };

console.log(Object.isFrozen(obj));  // false — before freeze
Object.freeze(obj);
console.log(Object.isFrozen(obj));  // true — after freeze
console.log(obj.x);                 // 10 — value preserved
