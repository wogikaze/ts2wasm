function f() {
  let source = "({ value: 7, label: 'ok' })";
  console.log(eval(source).value);
  console.log(eval(source).label);
  console.log(eval(source).missing);
}

f();
