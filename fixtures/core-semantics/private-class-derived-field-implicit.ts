class Base {
  constructor() {
    this.base = 3;
  }
}

class Derived extends Base {
  #value = 4;

  read() {
    return this.#value + this.base;
  }
}

let item = new Derived();
console.log(item.read());
