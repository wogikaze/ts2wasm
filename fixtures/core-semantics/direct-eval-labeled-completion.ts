function run() {
  let marker = 0;
  let result = eval("outer: { marker = 1; marker; break outer; marker = 2; marker; }");
  console.log(result);
  console.log(marker);
}

run();
