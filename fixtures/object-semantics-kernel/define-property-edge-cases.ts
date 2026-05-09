// W5.1: Property descriptor edge cases using getOwnPropertyDescriptor and freeze/seal
// Tests descriptor inspection for various object states

const obj = { a: 1, b: 2 };

// 1. Two properties, verify both descriptors
let da = Object.getOwnPropertyDescriptor(obj, "a");
let db = Object.getOwnPropertyDescriptor(obj, "b");
console.log(da.value + db.value);  // 3

// 2. Missing property returns undefined
let missing = Object.getOwnPropertyDescriptor(obj, "nonexistent");
console.log(missing === undefined ? "missing" : "found");

// 3. Object.create with explicit null prototype
const nullProto = Object.create(null);
console.log(Object.getPrototypeOf(nullProto) === null ? "null-proto" : "has-proto");

// 4. PreventExtensions: can't add new properties (verify via isExtensible)
const fixed = { existing: 1 };
console.log(Object.isExtensible(fixed));   // true
Object.preventExtensions(fixed);
console.log(Object.isExtensible(fixed));   // false
console.log(fixed.existing);               // 1 — existing property still works
