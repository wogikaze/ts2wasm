class Counter {
  static get #answer() {
    return 4;
  }

  static set #seen(value) {
    console.log(value + 1);
  }

  static read() {
    return this.#answer;
  }

  static readByName() {
    return Counter.#answer + 2;
  }

  static write(value) {
    this.#seen = value;
    return value;
  }

  static writeByName(value) {
    Counter.#seen = value;
    return value;
  }
}

console.log(Counter.read());
console.log(Counter.readByName());
console.log(Counter.write(8));
console.log(Counter.writeByName(10));
