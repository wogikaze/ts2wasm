// Array holes (sparse arrays)
// Exercises: holes at creation, holes after delete-like assignment

// Sparse array literal (holes)
let sparse = [1, , 3];
console.log(sparse.length);
console.log(sparse[0]);
console.log(sparse[1]);
console.log(sparse[2]);

// Create hole by assigning beyond length
let arr = [1, 2, 3];
arr[5] = 6;
console.log(arr.length);
console.log(arr[3]);
console.log(arr[4]);
console.log(arr[5]);

// Create hole by direct index skip
let a: number[] = [];
a[0] = 10;
a[2] = 30;
console.log(a.length);
console.log(a[0]);
console.log(a[1]);
console.log(a[2]);
