function run() {
  let result = eval("while (false) { var value = 2; } value");
  console.log(result);
  console.log(eval("value"));
}

run();
