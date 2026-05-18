function run() {
  let source = "var {value: created, nested: {leaf}} = {value: 4, nested: {leaf: 5}}; created + leaf";
  console.log(eval(source));
  console.log(created + leaf);
}

run();
