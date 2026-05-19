function run() {
  let key = "drop";
  let result = eval('var { [key]: removed, ...rest } = { drop: 1, keep: "ok" }; removed + ":" + rest.keep + ":" + rest.drop');
  console.log(result);
  console.log(removed);
  console.log(rest.keep);
  console.log(rest.drop);
}

run();
