const object: any = {
  [-0]: "negative zero",
  [Infinity]: "infinity",
  [-Infinity]: "negative infinity",
  [NaN]: "nan",
};

console.log(object[0]);
console.log(object[Infinity]);
console.log(object[-Infinity]);
console.log(object[NaN]);
