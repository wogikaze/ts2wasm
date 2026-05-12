// Array mutation — assignment to indices
// Exercises: index write, read back after write, overwrite

let arr = [1, 2, 3];

// Overwrite existing element
arr[0] = 99;
console.log(arr[0]);
console.log(arr[1]);
console.log(arr[2]);

// Extend array by assignment
arr[3] = 4;
console.log(arr.length);
console.log(arr[3]);

// Overwrite with different type
arr[1] = "hello";
console.log(arr[1]);

// Verify unchanged elements
console.log(arr[0]);
console.log(arr[2]);
