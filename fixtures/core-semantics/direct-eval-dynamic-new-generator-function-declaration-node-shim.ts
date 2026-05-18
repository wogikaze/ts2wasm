function run() {
  let source = "function* created() { yield 7; } created().next().value";
  let read = "created().next().value";
  console.log(eval(source));
  console.log(eval(read));
}

run();
