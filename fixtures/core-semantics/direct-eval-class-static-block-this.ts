function run() {
  let result = eval("class C { static { this.x = 7; } static value() { return this.x; } } C.value()");
  console.log(result);
  console.log(typeof C);
}

run();
