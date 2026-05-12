// Array pop method
// Exercises: pop, pop return value

let arr = [1, 2, 3];

let last = arr.pop();
console.log(last);
console.log(arr.length);
console.log(arr[2]);

// Pop until empty
let second = arr.pop();
console.log(second);
console.log(arr.length);

let first = arr.pop();
console.log(first);
console.log(arr.length);

// Pop from empty array
let empty: number[] = [];
let undef = empty.pop();
console.log(undef);
console.log(empty.length);
