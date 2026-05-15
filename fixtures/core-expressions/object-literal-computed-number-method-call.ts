function ID(x: number) {
  return x;
}

let object = {
  a() {
    return "A";
  },
  [1]() {
    return "B";
  },
  c() {
    return "C";
  },
  [ID(2)]() {
    return "D";
  },
};

console.log(object.a());
console.log(object[1]());
console.log(object.c());
console.log(object[2]());
