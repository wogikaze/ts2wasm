let outer = 5;
let object = {
  value: 7,
  method() {
    return this.value;
  },
  add(delta: number) {
    return this.value + delta + outer;
  },
};

console.log(object.method());
console.log(object.add(3));
