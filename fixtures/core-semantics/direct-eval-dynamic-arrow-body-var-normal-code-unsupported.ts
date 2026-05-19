function run() {
  let source = "let holder = () => { var hidden = 1; };";
  eval(source);
  console.log(hidden);
}

run();
