class Counter {
  #value = 2;

  #double() {
    return this.#value + this.#value;
  }

  #add(delta) {
    return this.#value + delta;
  }

  read() {
    return this.#double();
  }

  sum(delta) {
    return this.#add(delta);
  }
}

let counter = new Counter();
console.log(counter.read());
console.log(counter.sum(5));
