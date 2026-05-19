function run() {
  let source = "let holder = function hiddenProbe() { return 1; };";
  eval(source);
  console.log(hiddenProbe);
}

run();
