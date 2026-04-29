let pair = [4, 5];
let [a, b] = pair;
console.log(a);
console.log(b);

let obj = { x: 7, y: 8 };
let { x, y: renamed } = obj;
console.log(x);
console.log(renamed);

function combine([left], { x }) {
  return left + x;
}

let first = ([value]) => value;

console.log(combine([3], { x: 9 }));
console.log(first([11]));
