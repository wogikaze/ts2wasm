function run() {
  let result = eval("value(); function value() { return 4; }");
  console.log(result);
  console.log(eval("value()"));
}

run();
