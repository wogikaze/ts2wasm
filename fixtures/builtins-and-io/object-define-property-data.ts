// Object.defineProperty with data descriptor
const obj: any = {};

// Default attributes (all false per spec when not specified)
Object.defineProperty(obj, "a", { value: 42 });
const da = Object.getOwnPropertyDescriptor(obj, "a");
console.log(da.value);
console.log(da.writable);
console.log(da.enumerable);
console.log(da.configurable);

// Explicit attributes
Object.defineProperty(obj, "b", { value: 99, writable: false, enumerable: true, configurable: false });
const db = Object.getOwnPropertyDescriptor(obj, "b");
console.log(db.value);
console.log(db.writable);
console.log(db.enumerable);
console.log(db.configurable);

// Verify reading the value
console.log(obj.a);
console.log(obj.b);
