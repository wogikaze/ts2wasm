// W5.6: Property descriptor shapes via getOwnPropertyDescriptor
// Tests data descriptor shapes for literal and frozen properties

const obj = { x: 10 };

// Verify default data descriptor for literal property
let dx = Object.getOwnPropertyDescriptor(obj, "x");
console.log(dx.value);        // 10
console.log(dx.get === undefined ? "no-getter" : "has-getter");
console.log(dx.set === undefined ? "no-setter" : "has-setter");

// Freeze: all properties non-writable
Object.freeze(obj);
let dx2 = Object.getOwnPropertyDescriptor(obj, "x");
console.log(dx2.value);
console.log(dx2.writable);    // false

// Test basic property writing (sync field mutation)
const obj2 = { counter: 0 };
obj2.counter = obj2.counter + 1;
console.log(obj2.counter);    // 1
obj2.counter = obj2.counter + 1;
console.log(obj2.counter);    // 2
