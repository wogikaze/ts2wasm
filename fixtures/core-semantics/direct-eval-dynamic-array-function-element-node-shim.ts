let source = "[function add(a, b) { return a + b; }, function mul(a, b) { return a * b; }]";
let callbacks;
let first;
let second;
callbacks = eval(source);
first = callbacks[0];
second = callbacks[1];

console.log(first(2, 5));
console.log(second(3, 4));
