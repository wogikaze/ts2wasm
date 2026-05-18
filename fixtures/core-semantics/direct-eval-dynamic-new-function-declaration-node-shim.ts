function run() {
  let source = "function created() { return 7; } created()";
  let read = "created()";
  console.log(eval(source));
  console.log(eval(read));
}

run();
