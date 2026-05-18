function f() {
  let source = "err = err + 4; err";
  try {
    throw 3;
  } catch (err) {
    console.log(eval(source));
    console.log(err);
  }
}

f();
