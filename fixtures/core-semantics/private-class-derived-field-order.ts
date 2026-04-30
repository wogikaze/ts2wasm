function mark(prefix, suffix) {
  return prefix + suffix;
}

class Base {
  constructor() {
    this.trace = "base";
  }
}

class Derived extends Base {
  #value = mark(this.trace, "field");

  constructor() {
    super();
    this.trace = this.trace + "body";
  }

  read() {
    return this.#value;
  }
}

let item = new Derived();
console.log(item.trace);
console.log(item.read());
