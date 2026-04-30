function makeObj() {
  return { a: 1, b: 2 };
}

let source = makeObj();
let copy = { z: 0, ...source, b: 3 };

console.log(copy.z);
console.log(copy.a);
console.log(copy.b);
