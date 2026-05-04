// Array.prototype.at tests
// Test 1: basic positive index
let a: any = [10, 20, 30, 40, 50];
console.log(a.at(0));
console.log(a.at(2));
console.log(a.at(4));

// Test 2: negative index
console.log(a.at(-1));
console.log(a.at(-2));
console.log(a.at(-5));

// Test 3: out of bounds
console.log(a.at(10));
console.log(a.at(-10));

// Test 4: empty array
let b: any = [];
console.log(b.at(0));
console.log(b.at(-1));
