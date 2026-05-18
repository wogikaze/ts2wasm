function run() {
  let result = eval("function value() { return 4; } value()");
  console.log(result);
  console.log(eval("value()"));
}

run();
