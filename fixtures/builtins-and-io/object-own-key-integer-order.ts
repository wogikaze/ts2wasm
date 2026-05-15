function ID(x: number) {
  return x;
}

let object = {
  a() {
    return 1;
  },
  [10]() {
    return 10;
  },
  b() {
    return 2;
  },
  [ID(2)]() {
    return 20;
  },
  "01"() {
    return 1;
  },
  [1]() {
    return 100;
  },
};

console.log(Object.getOwnPropertyNames(object).join(","));
console.log(Object.keys(object).join(","));
