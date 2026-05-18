function f() {
  let value = 1;
  {
    let value = 2;
    let source = "value = 7; value";
    console.log(eval(source));
    console.log(value);
  }
  console.log(value);
}

f();
