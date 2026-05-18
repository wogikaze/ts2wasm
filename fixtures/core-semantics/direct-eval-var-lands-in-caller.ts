function run() {
  let result = eval("var value = 2; value");
  console.log(result);
  console.log(eval("value"));
}

run();
