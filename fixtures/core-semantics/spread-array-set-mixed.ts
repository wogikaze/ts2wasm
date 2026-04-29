let values = [2, 1, 2, 3];
let set = new Set(values);
let copy = [0, ...set, 4];

console.log(copy.length);
console.log(copy[0]);
console.log(copy[1]);
console.log(copy[2]);
console.log(copy[3]);
