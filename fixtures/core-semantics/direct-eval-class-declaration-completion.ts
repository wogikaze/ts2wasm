function run() {
  let result = eval("class C { value() { return 5; } } let kind = typeof C; kind + ':' + new C().value()");
  console.log(result);
  console.log(typeof C);
}

run();
