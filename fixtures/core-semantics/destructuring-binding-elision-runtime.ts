let pair = [1, 2, 3];
let [, second] = pair;
console.log(second);

let [, defaulted = 5] = [0];
console.log(defaulted);

function pickSecond([, value]) {
  return value;
}

let pickThird = ([, , value]) => value;

console.log(pickSecond([7, 8]));
console.log(pickThird([9, 10, 11]));
