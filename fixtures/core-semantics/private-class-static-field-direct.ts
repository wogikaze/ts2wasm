class C {
  static #value = 1;

  static read() {
    return this.#value;
  }

  static write(value) {
    this.#value = value;
    return C.#value;
  }
}

console.log(C.read());
console.log(C.write(2));
