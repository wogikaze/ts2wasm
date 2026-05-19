function run() {
  let result = eval("for (var key in { alpha: 1 }) {} for (var value of [4]) {} key + ':' + value");
  console.log(result);
  console.log(key);
  console.log(value);
}

run();
