// Closures / nested functions capturing outer scope
function makeCounter(): () => number {
  let count = 0;
  function increment(): number {
    count = count + 1;
    return count;
  }
  return increment;
}

let counter = makeCounter();
console.log(counter().toString());
console.log(counter().toString());
console.log(counter().toString());

function makeAdder(x: number): (n: number) => number {
  return function(y: number): number {
    return x + y;
  };
}

let add5 = makeAdder(5);
console.log(add5(3).toString());
console.log(add5(7).toString());
