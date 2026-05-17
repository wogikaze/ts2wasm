const input = "hello world hello";
const matches = input.matchAll("hello");
const array = [...matches];

console.log(array.length);
console.log(array[0][0]);
console.log(array[0].index);
console.log(array[0].input);
console.log(array[1][0]);
console.log(array[1].index);
