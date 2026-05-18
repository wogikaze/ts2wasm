function f() {
  let value = 1;
  let source = '"use strict"; let value = 2; value';
  console.log(eval(source));
  console.log(value);
}

f();
