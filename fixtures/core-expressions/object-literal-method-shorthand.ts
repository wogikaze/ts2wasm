// Object literal method shorthand
const obj = {
  name: "test",
  greet() {
    return "Hello " + this.name;
  },
  double(n: number) {
    return n * 2;
  }
};

console.log(obj.greet());
console.log(obj.double(21));
