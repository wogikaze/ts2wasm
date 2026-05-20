// Test: Function.prototype.call on runtime function values (local variables)

function add(a, b) {
  return a + b;
}

function addBase(value) {
  return this.base + value;
}

const ctx = { base: 10 };

// Function stored in a local variable
let localFn = add;
console.log(localFn.call(undefined, 1, 2));
console.log(localFn.call(undefined, 10, 20));

// Function with thisArg via call on local
let localThisFn = addBase;
console.log(localThisFn.call(ctx, 5));
console.log(localThisFn.call(ctx, 15));

// Function.prototype.apply on local
console.log(localFn.apply(undefined, [3, 4]));
console.log(localThisFn.apply(ctx, [7]));

// No args call
function fortyTwo() { return 42; }
let f = fortyTwo;
console.log(f.call(undefined));

// Multiple args via call
console.log(localFn.call(undefined, 100, 200));
