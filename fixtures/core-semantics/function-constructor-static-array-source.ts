let single = Function(["return 7"]);
let joined = Function(['console.log("x', 'y")']);
let empty = Function([]);

console.log(single());
console.log(joined());
console.log(joined.toString());
console.log(empty());
console.log(empty.toString());
