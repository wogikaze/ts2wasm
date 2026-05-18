function run() {
  var value;
  let source = "var value = 7; value";
  let read = "value";
  console.log(eval(source));
  console.log(eval(read));
}

run();
