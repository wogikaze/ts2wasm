// Method override in subclass
class Base {
  greet() {
    return "Hello from Base";
  }
  double(n: number) {
    return n * 2;
  }
}

class Derived extends Base {
  greet() {
    return "Hello from Derived";
  }
  triple(n: number) {
    return n * 3;
  }
}

const b = new Base();
const d = new Derived();

console.log(b.greet());
console.log(d.greet());
console.log(b.double(5));
console.log(d.double(5));
console.log(d.triple(5));
