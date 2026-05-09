// W5.4: Freeze descriptor interaction via getOwnPropertyDescriptor
// Object.freeze makes properties non-writable and non-configurable

const obj = { a: 1, b: 2, c: 3 };

// 1. Initial descriptors (literal defaults)
let da = Object.getOwnPropertyDescriptor(obj, "a");
console.log(da.value);
console.log(da.writable);     // true
console.log(da.configurable); // true

// 2. Freeze: all properties non-writable, non-configurable
Object.freeze(obj);
let da2 = Object.getOwnPropertyDescriptor(obj, "a");
console.log(da2.value);
console.log(da2.writable);     // false
console.log(da2.configurable); // false

// 3. Write to frozen object: silently rejected
obj.a = 99;
console.log(obj.a);           // 1 — unchanged

// 4. Freeze vs frozen check
console.log(Object.isFrozen(obj));  // true

// 5. Property keys still accessible after freeze
let keys = Object.keys(obj);
console.log(keys.length);     // 3
