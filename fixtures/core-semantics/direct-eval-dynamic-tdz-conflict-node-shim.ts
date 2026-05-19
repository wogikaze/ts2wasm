// Supported host-lane boundary: descriptor v2 marks `value` as TDZ, so runtime
// direct eval reports a catchable ReferenceError instead of a build-time
// UnsupportedEval diagnostic.
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
