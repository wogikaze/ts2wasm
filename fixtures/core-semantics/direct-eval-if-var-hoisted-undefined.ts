function run() {
  let result = eval("if (false) { var value = 2; } value");
  console.log(result);
  console.log(eval("value"));
}

run();
