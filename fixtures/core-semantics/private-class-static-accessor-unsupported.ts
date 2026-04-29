class C {
  static get #value() {
    return 1;
  }

  static set #value(next) {
    console.log(next);
  }

  static read() {
    return this.#value;
  }

  static write(next) {
    this.#value = next;
    return C.#value;
  }
}

console.log(C.read());
console.log(C.write(2));
