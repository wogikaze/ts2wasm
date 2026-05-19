function run() {
  let result = eval("if (false) { var { item } = { item: 6 }; } typeof item");
  console.log(result);
  console.log(item);
}

run();
