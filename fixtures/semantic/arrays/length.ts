// Array length property
// Exercises: .length read, length after mutation

let arr = [1, 2, 3];
console.log(arr.length);

// Length after push
arr.push(4);
console.log(arr.length);

// Length after pop
arr.pop();
console.log(arr.length);

// Length after assignment beyond current length
arr[10] = 99;
console.log(arr.length);

// Empty array length
let empty: number[] = [];
console.log(empty.length);

// Single element
let single = [42];
console.log(single.length);
