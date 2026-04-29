let source = { x: 1, y: 2 };
let { x, ...rest } = source;
console.log(x);
console.log(rest.y);
