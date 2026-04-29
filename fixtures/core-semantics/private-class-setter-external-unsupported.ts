class C {
  set #x(value) {}
}

let c = new C();
c.#x = 1;
console.log(c);
