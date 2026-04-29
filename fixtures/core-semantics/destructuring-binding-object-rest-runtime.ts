let { a, ...rest } = { a: 1, b: 2, c: 3 };
console.log(a);
console.log(rest.b);
console.log(rest.c);
console.log(rest.a);

let { left: renamed, ...tail } = { left: 4, right: 5, extra: 6 };
console.log(renamed);
console.log(tail.right);
console.log(tail.extra);
