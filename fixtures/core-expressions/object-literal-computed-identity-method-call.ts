function id(x: string) {
  return x;
}

let obj = {
  a() {
    return "A";
  },
  [id("d")]() {
    return "D";
  },
};

console.log(obj.a());
console.log(obj.d());
