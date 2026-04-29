let missing = null;
let absent = undefined;

function add(left, right) {
  return left + right;
}

let twice = x => x * 2;

console.log(missing?.());
console.log(absent?.());
console.log(add?.(2, 3));
console.log(twice?.(6));
