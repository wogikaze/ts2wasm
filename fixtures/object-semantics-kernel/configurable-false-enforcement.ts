// W5.4: Freeze behavior — isFrozen after freeze
// Note: per-property configurable enforcement is not yet implemented

const obj = { a: 1, b: 2 };

console.log(Object.isFrozen(obj));  // false — before freeze
Object.freeze(obj);
console.log(Object.isFrozen(obj));  // true — after freeze
