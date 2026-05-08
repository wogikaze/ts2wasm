// Cover initializer — destructuring with default in parenthesized expression
let a, b;
({ a = 1, b = 2 } = { a: 10, b: 20 });
console.log(a, b);
