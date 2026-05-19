function run() {
  let result = eval("for (var x = 1; false;) {} x");
  console.log(result);
  console.log(x);
}

run();
