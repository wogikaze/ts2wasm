// HIR-lowerable builtin calls
// These builtins can be lowered through the HIR path.

console.log(Math.abs(-5));
console.log(Math.abs(0));
console.log(Math.abs(3));

console.log(Number(42));
console.log(Number("123"));
console.log(Number(true));

console.log(Array.isArray([1, 2, 3]));
console.log(Array.isArray({}));
console.log(Array.isArray("hello"));

console.log(Math.ceil(3.1));
console.log(Math.floor(3.9));
