let positive = Function(+"2.5");
let negative = Function(-"1.5");
let empty = Function(+"");

console.log(positive());
console.log(negative());
console.log(empty());
console.log(positive.toString());
console.log(negative.toString());
console.log(empty.toString());
