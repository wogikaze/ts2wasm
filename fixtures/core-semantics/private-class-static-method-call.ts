class Counter {
  static #value() {
    return 4;
  }

  static #add(delta) {
    return delta + 3;
  }

  static read() {
    return this.#value();
  }

  static sum(delta) {
    return Counter.#add(delta);
  }
}

console.log(Counter.read());
console.log(Counter.sum(5));
