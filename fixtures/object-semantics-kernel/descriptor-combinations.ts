// W5.1+W5.4: Property descriptor introspection via getOwnPropertyDescriptor
// Tests default attributes for literal properties before freeze

const obj = { a: 1, b: 2 };

// 1. Literal property defaults: writable, enumerable, configurable = true
let da = Object.getOwnPropertyDescriptor(obj, "a");
console.log(da.value);
console.log(da.writable);     // true
console.log(da.enumerable);   // true
console.log(da.configurable); // true

// 2. PreventExtensions: does not change descriptor flags
const obj2 = { x: "extensible" };
Object.preventExtensions(obj2);
let dx = Object.getOwnPropertyDescriptor(obj2, "x");
console.log(dx.value);
console.log(dx.configurable); // true
console.log(dx.writable);     // true
