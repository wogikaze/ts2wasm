class C {
  #value = 1;

  read() {
    return this.#value;
  }
}

let c = new C();
delete c["__ts2wasm_private::C::value"];
console.log(c.read());
