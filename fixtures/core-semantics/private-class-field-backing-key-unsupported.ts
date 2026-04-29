class Counter {
  #value = 7;

  read() {
    return this.#value;
  }
}

let c = new Counter();
console.log(c["__ts2wasm_private::Counter::value"]);
