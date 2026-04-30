class Base {
  constructor() {
    this.base = 4;
  }
}

class Derived extends Base {
  constructor() {
    super();
  }

  #add(value) {
    return this.base + value;
  }

  read() {
    return this.#add(3);
  }
}

let item = new Derived();
console.log(item.read());
