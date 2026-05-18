function run() {
  let seen = "";
  let result = eval('let obj = { a: 1, b: 2 }; for (let key in obj) { seen = seen + key; seen; }');
  console.log(result);
  console.log(seen);
}

run();
