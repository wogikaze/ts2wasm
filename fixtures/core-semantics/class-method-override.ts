// Class method override via extends
class Base {
  greet() {
    return "Hello from Base";
  }
}

class Derived extends Base {
  greet() {
    return "Hello from Derived";
  }
}

const b = new Base();
const d = new Derived();
console.log(b.greet());
console.log(d.greet());
