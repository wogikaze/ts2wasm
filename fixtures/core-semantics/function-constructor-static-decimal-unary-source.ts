let negative = Function(-1.5);
let positive = new Function(+2.5);

console.log(negative());
console.log(positive());
console.log(negative.toString());
console.log(positive.toString());
