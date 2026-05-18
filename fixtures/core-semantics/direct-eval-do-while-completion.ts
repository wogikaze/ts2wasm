function run() {
  let i = 0;
  let result = eval("do { i = i + 1; i; } while (i < 3);");
  console.log(result);
  console.log(i);
}

run();
