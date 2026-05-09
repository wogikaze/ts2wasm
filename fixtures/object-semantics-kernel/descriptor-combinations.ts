// W5.1+W5.4: Property descriptor introspection via getOwnPropertyDescriptor
// Tests default attributes for literal properties

const obj = { a: 1, b: 2 };

// 1. Literal property defaults: writable, enumerable, configurable = true
let da = Object.getOwnPropertyDescriptor(obj, "a");
console.log(da.value);
console.log(da.writable);     // true
console.log(da.enumerable);   // true
console.log(da.configurable); // true

// 2. Freeze: all properties non-writable, non-configurable
Object.freeze(obj);
let da2 = Object.getOwnPropertyDescriptor(obj, "a");
console.log(da2.value);
console.log(da2.writable);    // false
console.log(da2.configurable); // false
