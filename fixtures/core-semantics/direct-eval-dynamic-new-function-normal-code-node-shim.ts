function run() {
  let source = "function created() { return 7; } created()";
  console.log(eval(source));
  console.log(created());
}

run();
