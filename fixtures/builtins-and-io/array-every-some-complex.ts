// Array every/some with complex predicates
const arr = [1, 2, 3, 4, 5];
console.log(arr.every(x => x > 0));
console.log(arr.every(x => x > 3));
console.log(arr.some(x => x === 3));
console.log(arr.some(x => x > 10));

// With index argument
console.log(arr.every((x, i) => x === i + 1));
console.log(arr.some((x, i) => x !== i + 1));
