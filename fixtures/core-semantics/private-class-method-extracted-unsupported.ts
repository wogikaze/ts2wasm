class C {
  #m() {
    return 1;
  }

  read() {
    let f = this.#m;
    return f();
  }
}

let c = new C();
console.log(c.read());
