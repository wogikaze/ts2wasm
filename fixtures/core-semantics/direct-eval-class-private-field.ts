function run() {
  let result = eval("class C { #value = 8; value() { return this.#value; } } new C().value()");
  console.log(result);
  console.log(typeof C);
}

run();
