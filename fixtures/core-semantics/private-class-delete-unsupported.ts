class C {
  #value = 1;

  clear() {
    return delete this.#value;
  }
}

let c = new C();
console.log(c.clear());
