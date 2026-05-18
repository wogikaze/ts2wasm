function run() {
  let result = eval("let { value } = { value: 3 }; let [first] = [4]; value + first");
  console.log(result);
}

run();
