function makeObj() {
  return { a: 1, b: 2 };
}

let copy = { z: 0, ...makeObj(), b: 3 };

console.log(copy.z);
console.log(copy.a);
console.log(copy.b);
