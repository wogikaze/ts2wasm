function run() {
  let key = "value";
  eval('for (var { [key]: item } of [{ value: "ok" }]) {}');
  console.log(item);
}

run();
