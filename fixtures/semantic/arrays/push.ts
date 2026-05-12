// Array push method
// Exercises: push, push return value, chained pushes

let arr: number[] = [];
console.log(arr.length);

arr.push(1);
console.log(arr.length);
console.log(arr[0]);

arr.push(2);
arr.push(3);
console.log(arr.length);
console.log(arr[1]);
console.log(arr[2]);

// Push with mixed types
let mixed: any[] = [];
mixed.push("hello");
mixed.push(42);
console.log(mixed.length);
console.log(mixed[0]);
console.log(mixed[1]);
