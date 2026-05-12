// Argument count edge cases: testing exact-arg calls that match Node output

function add(a: number, b: number): number {
  return a + b;
}
console.log(add(3, 4));
console.log(add(10, 20));

function echo(x: number): number {
  return x;
}
console.log(echo(1));
console.log(echo(42));

function sum3(a: number, b: number, c: number): number {
  return a + b + c;
}
console.log(sum3(1, 2, 3));
console.log(sum3(10, 20, 30));

function noParams(): number {
  return 99;
}
console.log(noParams());
