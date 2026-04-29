let empty = [];
let [arrayDefault, arrayKept = 2] = [undefined, 9];
console.log(arrayDefault);
console.log(arrayKept);

let obj = { present: 7 };
let { missing = 3, present: renamed = 4 } = obj;
console.log(missing);
console.log(renamed);

function pick([value = 5], { x = 6 }) {
  return value + x;
}

function fallback([value] = [8]) {
  return value;
}

let arrowPick = ([value = 10]) => value;

console.log(pick(empty, {}));
console.log(fallback());
console.log(arrowPick([]));
