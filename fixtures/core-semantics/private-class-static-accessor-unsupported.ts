class C {
  static get #value() {
    return 1;
  }

  static set #next(next) {
    console.log(next);
  }
}

console.log(C.#value);
