let less = Function(1 < 2);
let greater = Function(3 > 4);
let strictEqual = Function(2 === 2);
let looseEqual = Function("2" == 2);
let notEqual = Function(null != undefined);
let stringOrder = Function("a" < "b");

console.log(less());
console.log(greater());
console.log(strictEqual());
console.log(looseEqual());
console.log(notEqual());
console.log(stringOrder());
console.log(less.toString());
console.log(greater.toString());
console.log(strictEqual.toString());
console.log(looseEqual.toString());
console.log(notEqual.toString());
console.log(stringOrder.toString());
