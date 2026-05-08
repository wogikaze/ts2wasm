// Object.assign copies values (always creates writable data descriptors)
const target: any = {};
Object.defineProperty(target, "x", { value: 1, writable: true, enumerable: true, configurable: true });
const source = { y: 2 };
Object.assign(target, source);
console.log(target.x);
console.log(target.y);

// Verify descriptor attributes after assign
const dx = Object.getOwnPropertyDescriptor(target, "x");
console.log(dx.value);
console.log(dx.writable);
console.log(dx.enumerable);
console.log(dx.configurable);

const dy = Object.getOwnPropertyDescriptor(target, "y");
console.log(dy.value);
console.log(dy.writable);
console.log(dy.enumerable);
console.log(dy.configurable);
