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

const boundSum = sum3.bind(undefined, 1);
console.log(boundSum(2, 3));

const boundBase = addBase.bind(ctx, 6);
console.log(boundBase());

const sumCall = Function.prototype.call.bind(sum3);
console.log(sumCall(undefined, 2, 4, 6));

const sumApply = Function.prototype.apply.bind(sum3);
console.log(sumApply(undefined, [3, 6, 9]));

const reboundSum = Function.prototype.bind.call(sum3, undefined, 4, 5);
console.log(reboundSum(6));
