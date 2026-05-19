function run() {
  let key = "drop";
  eval('for (var { [key]: removed, ...rest } of [{ drop: 1, keep: "ok" }]) {}');
  console.log(removed);
  console.log(rest.keep);
  console.log(rest.drop);
}

run();
