function run() {
  eval('for (var { ["value"]: item } of [{ value: "ok" }]) {}');
  console.log(item);
}

run();
