// Basic function declaration and calling
function greet(): void {
  console.log("hello");
}
greet();

function add(a: number, b: number): number {
  return a + b;
}
console.log(add(1, 2).toString());

function echo(x: number): number {
  return x;
}
console.log(echo(42).toString());
