function f() {
  let source = "throw new Error('direct boom')";
  try {
    eval(source);
  } catch {
    console.log("caught direct boom");
  }
}

f();
