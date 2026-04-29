class C {
  get #x() {
    return 1;
  }
}

let c = new C();
console.log(c.#x);
