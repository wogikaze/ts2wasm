function f() {
  let source = "throw new Error('direct boom')";
  try {
    eval(source);
  } catch (err) {
    console.log(err.name);
    console.log(err.message);
  }
}

f();
