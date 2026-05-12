// Function expressions assigned to variables
let multiply = function(a: number, b: number): number {
  return a * b;
};
console.log(multiply(3, 4).toString());

let greetFn = function(name: string): string {
  return "hello " + name;
};
console.log(greetFn("world"));
