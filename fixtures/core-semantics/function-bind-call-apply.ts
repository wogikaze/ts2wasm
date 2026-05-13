function sum3(a, b, c) {
  return a + b + c;
}

function addBase(value) {
  return this.base + value;
}

const ctx = { base: 10 };

console.log(sum3.call(undefined, 1, 2, 3));
console.log(sum3.apply(undefined, [4, 5, 6]));
console.log(sum3.bind(undefined, 7)(8, 9));
console.log(addBase.call(ctx, 3));
console.log(addBase.apply(ctx, [4]));
console.log(addBase.bind(ctx)(5));
