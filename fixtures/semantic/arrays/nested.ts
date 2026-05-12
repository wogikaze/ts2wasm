// Nested arrays
// Exercises: array of arrays, index access into nested arrays

let nested = [[1, 2], [3, 4], [5, 6]];
console.log(nested.length);
console.log(nested[0][0]);
console.log(nested[0][1]);
console.log(nested[1][0]);
console.log(nested[1][1]);
console.log(nested[2][0]);
console.log(nested[2][1]);

// Variable index into nested
let outer = 1;
let inner = 0;
console.log(nested[outer][inner]);

// Mutate nested
nested[0][1] = 99;
console.log(nested[0][0]);
console.log(nested[0][1]);
