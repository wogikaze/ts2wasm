function f() {
  let value = "before";
  let source = "value = 'after'; value";
  console.log(eval(source));
  console.log(value);
}

f();
