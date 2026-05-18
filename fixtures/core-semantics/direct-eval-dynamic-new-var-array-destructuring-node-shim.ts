function run() {
  let source = "var [first, , ...rest] = [2, 0, 3, 4]; first + rest.length";
  let read = "first + rest.length";
  console.log(eval(source));
  console.log(eval(read));
}

run();
