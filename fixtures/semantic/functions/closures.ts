// Closures / nested functions capturing outer scope (read-only captures only)

function makeAdder(x: number): (n: number) => number {
  return function(y: number): number {
    return x + y;
  };
}

let add5 = makeAdder(5);
console.log(add5(3));
console.log(add5(7));

function makeMultiplier(factor: number): (n: number) => number {
  function multiply(n: number): number {
    return n * factor;
  }
  return multiply;
}

let double = makeMultiplier(2);
console.log(double(10));
console.log(double(21));
