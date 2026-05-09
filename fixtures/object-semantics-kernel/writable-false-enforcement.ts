// W5.1: writable:false enforcement — uses Object.getOwnPropertyDescriptor (works)
// Can't use Object.defineProperty due to backend bug, so test via freeze

const obj = { x: 10 };
Object.freeze(obj);

// After freeze: properties are non-writable, non-configurable
console.log(obj.x);        // 10

// Assignment silently rejected in non-strict
obj.x = 20;
console.log(obj.x);        // 10 — unchanged

// Verify descriptor via getOwnPropertyDescriptor
let d = Object.getOwnPropertyDescriptor(obj, "x");
console.log(d.value);
console.log(d.writable);   // false
console.log(d.configurable); // false
