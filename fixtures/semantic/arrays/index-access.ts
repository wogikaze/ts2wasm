// Array index access — read and write by index
// Exercises: arr[i] read/write, variable index, out-of-bounds
// Note: arr[0] was fixed in #390 (P14 regression)

let arr = [10, 20, 30, 40, 50];

// Read by literal index
console.log(arr[0]);
console.log(arr[2]);
console.log(arr[4]);

// Read by variable index
let i = 1;
console.log(arr[i]);

let j = 3;
console.log(arr[j]);

// Write by literal index
arr[1] = 25;
console.log(arr[1]);

// Write by variable index
let k = 3;
arr[k] = 45;
console.log(arr[k]);

// Write and read back
arr[0] = 99;
console.log(arr[0]);

// Out of bounds (returns undefined)
console.log(arr[10]);
console.log(arr[-1]);
