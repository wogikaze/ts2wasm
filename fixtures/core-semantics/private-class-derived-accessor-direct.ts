class Base {
  constructor() {
    this.base = 10;
  }
}

class Derived extends Base {
  #value = 1;

  constructor() {
    super();
  }

  set #next(value) {
    this.#value = value;
  }

  get #current() {
    return this.#value;
  }

  write(value) {
    this.#next = value;
    return this.#current + this.base;
  }
}

let item = new Derived();
console.log(item.write(5));
console.log(item.write(8));
