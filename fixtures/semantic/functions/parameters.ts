// Function parameters
function sum3(a: number, b: number, c: number): number {
  return a + b + c;
}
console.log(sum3(1, 2, 3).toString());

function concat(a: string, b: string): string {
  return a + b;
}
console.log(concat("foo", "bar"));

function repeat(s: string, n: number): string {
  let r = "";
  for (let i = 0; i < n; i++) {
    r = r + s;
  }
  return r;
}
console.log(repeat("x", 4));
