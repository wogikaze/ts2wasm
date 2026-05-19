function run() {
  let source = "/later/.test('later')";
  let result = eval(source);
  let later = "after";
  console.log(result);
  console.log(later);
}

run();
