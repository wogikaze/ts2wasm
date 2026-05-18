function run() {
  let total = 0;
  let result = eval("for (let value of [1, 2, 3]) { total = total + value; total; }");
  console.log(result);
  console.log(total);
}

run();
