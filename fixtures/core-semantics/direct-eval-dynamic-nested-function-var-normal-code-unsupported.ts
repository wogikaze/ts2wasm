function run() {
  let source = "function outer() { var hidden = 1; function inner() { return hidden; } }";
  eval(source);
  console.log(hidden);
}

run();
