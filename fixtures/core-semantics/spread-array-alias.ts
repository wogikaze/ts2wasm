let base = [3, 4, 5];
let values = base;

function sum(a, b, c) {
  return a + b + c;
}

let copy = [0, ...values, 6];

console.log(copy.length);
console.log(copy[0]);
console.log(copy[1]);
console.log(copy[2]);
console.log(copy[3]);
console.log(copy[4]);
console.log(sum(...values));
