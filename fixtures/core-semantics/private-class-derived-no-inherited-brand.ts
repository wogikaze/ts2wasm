class Base {
  constructor() {
    this.base = "base";
  }
}

class Derived extends Base {
  #value = 9;

  constructor() {
    super();
    this.derived = "derived";
  }

  readFrom(other) {
    try {
      other.#value;
      console.log("ok");
    } catch (e) {
      console.log("caught");
    }
  }
}

let derived = new Derived();
let base = new Base();

derived.readFrom(derived);
derived.readFrom(base);
console.log(derived.base);
console.log(derived.derived);
