function run() {
  var value = 4;
  var getValue;
  let source = "function getValue() { return value + 3; } getValue()";
  let read = "getValue()";
  console.log(eval(source));
  console.log(eval(read));
}

run();
