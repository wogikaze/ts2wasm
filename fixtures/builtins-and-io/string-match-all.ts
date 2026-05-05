const matches = "matchAll".matchAll(/\w/g);
const array = [...matches];

console.log(array.length);
console.log(array[0][0]);
console.log(array[0].index);
console.log(array[0].input);
console.log(array[5][0]);
console.log(array[5].index);
