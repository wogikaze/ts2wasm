function run() {
  let source = "for (var key in { alpha: 1 }) {} for (var value of [4]) {}";
  eval(source);
  console.log(key);
  console.log(value);
}

run();
