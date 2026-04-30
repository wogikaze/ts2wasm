class Counter {
  static #trace = "field-a";

  static {
    console.log(Counter.#trace);
    Counter.#trace = Counter.#trace + ":block-a";
  }

  static #tail = "field-b";

  static {
    console.log(Counter.#trace + ":" + Counter.#tail);
    Counter.#tail = Counter.#tail + ":block-b";
  }

  static read() {
    return Counter.#trace + ":" + Counter.#tail;
  }
}

console.log(Counter.read());
