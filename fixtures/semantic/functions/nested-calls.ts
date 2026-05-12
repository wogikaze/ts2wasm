// Nested function calls (composition)
function double(x: number): number {
  return x * 2;
}
function addOne(x: number): number {
  return x + 1;
}
function square(x: number): number {
  return x * x;
}

// Nesting calls
console.log(double(addOne(4)));
console.log(square(double(3)));
console.log(addOne(square(double(1))));
