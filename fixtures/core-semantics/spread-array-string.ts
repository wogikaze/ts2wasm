let literal = [..."ab"];
let letters = "cd";
let copy = letters;
let tail = ["e"];
let values = [0, ...literal, ...copy, ...tail, "f"];

console.log(values.length);
console.log(values[0]);
console.log(values[1]);
console.log(values[2]);
console.log(values[3]);
console.log(values[4]);
console.log(values[5]);
console.log(values[6]);
