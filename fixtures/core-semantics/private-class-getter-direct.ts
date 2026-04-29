class Counter {
  #value = 4;

  get #double() {
    return this.#value + this.#value;
  }

  read() {
    return this.#double;
  }
}

let counter = new Counter();
console.log(counter.read());
console.log(counter.read() + 1);
