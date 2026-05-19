function run() {
  let source = "let obj = { method() { var hidden = 1; } };";
  eval(source);
  console.log(hidden);
}

run();
