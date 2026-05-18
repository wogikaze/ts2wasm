function run(value) {
  let result = eval("arguments[0] + ':' + arguments.length");
  console.log(result);
}

run(7, 8, 9);
