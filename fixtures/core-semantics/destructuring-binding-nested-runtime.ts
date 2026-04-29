let nested = [[1, 2], [3, 4]];
let [[first, second], [third]] = nested;
console.log(first);
console.log(second);
console.log(third);

function pick([[value]]) {
  return value;
}

let sumPair = ([[left, right]]) => left + right;

console.log(pick([[7]]));
console.log(sumPair([[5, 6]]));
