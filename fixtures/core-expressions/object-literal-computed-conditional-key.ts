const truthy: any = {
  [true ? 1 : 2]: "one",
};
const falsy: any = {
  [false ? 1 : 2]: "two",
};

console.log(truthy[true ? 1 : 2]);
console.log(truthy[String(true ? 1 : 2)]);
console.log(falsy[false ? 1 : 2]);
console.log(falsy[String(false ? 1 : 2)]);
