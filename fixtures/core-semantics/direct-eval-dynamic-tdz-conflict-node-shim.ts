function f() {
  let source = "value";
  try {
    eval(source);
  } catch (error) {
    console.log(error.name);
  }
  let value = 1;
  console.log("after");
}

f();
