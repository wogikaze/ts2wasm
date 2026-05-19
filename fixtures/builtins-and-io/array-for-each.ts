// Array.prototype.forEach with callback collecting side effects
// Validates that forEach actually invokes the callback.

// Test 1: forEach with arrow function, collect elements into a result array
let source = [10, 20, 30];
let result: number[] = [];
let indices: number[] = [];

source.forEach((v, i) => {
  result.push(v);
  indices.push(i);
});

console.log(result[0]);
console.log(result[1]);
console.log(result[2]);
console.log(indices[0]);
console.log(indices[1]);
console.log(indices[2]);

// Test 2: forEach with function expression callback
let doubled: number[] = [];
source.forEach(function (v) {
  doubled.push(v * 2);
});

console.log(doubled[0]);
console.log(doubled[1]);
console.log(doubled[2]);

// Test 3: forEach on empty array (callback should not be called)
let empty: number[] = [];
let called = false;
empty.forEach(() => {
  called = true;
});
let check = called ? 0 : 1;
console.log(check);

// Test 4: forEach with index and array args
let selfCheck: number[] = [];
source.forEach((v, i, arr) => {
  selfCheck.push(arr[i]);
});
console.log(selfCheck[0]);
console.log(selfCheck[1]);
console.log(selfCheck[2]);

// Test 5: forEach with side effect on outer variable
let sum = 0;
source.forEach((v) => {
  sum = sum + v;
});
console.log(sum);
