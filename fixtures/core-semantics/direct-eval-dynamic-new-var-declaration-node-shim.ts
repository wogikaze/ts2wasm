function run() {
  let source = "var created = 7; created";
  let read = "created";
  console.log(eval(source));
  console.log(eval(read));
}

run();
