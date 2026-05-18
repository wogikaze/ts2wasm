let body = "return [function add(a, b) { return a + b; }, function mul(a, b) { return a * b; }]";
let make = Function(body);
let callbacks = make();
let add = callbacks[0];
let mul = callbacks[1];

console.log(add(2, 5));
console.log(mul(3, 4));
