class Counter {
  #value = 1;

  set #next(value) {
    this.#value = value;
  }

  get #current() {
    return this.#value;
  }

  write(value) {
    this.#next = value;
    return this.#current;
  }
}

let counter = new Counter();
console.log(counter.write(5));
console.log(counter.write(8));
