// Argument count edge cases: extra args, missing args behavior

function add(a: number, b: number): number {
  return a + b;
}

// Normal call
console.log(add(3, 4));

// Extra args are silently ignored
console.log(add(3, 4, 5));
console.log(add(3, 4, 5, 6, 7));

// Single-param function, extra args ignored
function echo(x: number): number {
  return x;
}
console.log(echo(1));
console.log(echo(1, 2));
console.log(echo(1, 2, 3));

// Three-param function
function sum3(a: number, b: number, c: number): number {
  return a + b + c;
}
console.log(sum3(1, 2, 3));
console.log(sum3(1, 2, 3, 4));
console.log(sum3(1, 2, 3, 4, 5));
