function run() {
  let source = "async function created() { return 7; } created.constructor.name";
  let read = "created.constructor.name";
  console.log(eval(source));
  console.log(eval(read));
}

run();
