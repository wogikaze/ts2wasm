function run() {
  let source = "var [first, , ...rest] = [2, 0, 3, 4]; first + rest.length";
  console.log(eval(source));
  console.log(first + rest.length);
}

run();
