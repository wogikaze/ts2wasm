let zero = { x: 0n };
let one = { x: 1n };
let two = { x: 2n };
let neg = { x: -1n };
let truthy = { x: true };
let falsy = { x: false };

console.log(one.x < truthy.x);
console.log(zero.x <= falsy.x);
console.log(two.x > truthy.x);
console.log(neg.x < falsy.x);
console.log(falsy.x < one.x);
console.log(truthy.x <= one.x);
console.log(truthy.x > zero.x);
console.log(one.x >= truthy.x);
console.log(falsy.x >= zero.x);
console.log(neg.x >= falsy.x);
