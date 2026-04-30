class Counter {
  static {
    console.log(Counter.#later);
  }

  static #later = 1;
}
