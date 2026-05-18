function run() {
  let tag = 2;
  let seen = 0;
  let result = eval("switch (tag) { case 1: seen = 10; seen; break; case 2: seen = 20; seen; break; default: seen = 30; seen; }");
  console.log(result);
  console.log(seen);
}

run();
