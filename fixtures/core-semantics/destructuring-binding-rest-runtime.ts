let pair = [1, 2, 3, 4];
let [first, ...tail] = pair;
console.log(first);
console.log(tail.length);
console.log(tail[0]);
console.log(tail[2]);

let [only, ...empty] = [9];
console.log(only);
console.log(empty.length);

function restLength([head, ...rest]) {
  return head + rest.length + rest[1];
}

let pickRest = ([, ...rest]) => rest[0] + rest.length;

console.log(restLength([5, 6, 7]));
console.log(pickRest([10, 11, 12]));
