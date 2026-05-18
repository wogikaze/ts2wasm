function run() {
  let i = 0;
  let result = eval("while (i < 3) { i = i + 1; i; }");
  console.log(result);
  console.log(i);
}

run();
