// try/catch basic
try {
  let x = 1;
  console.log(x);
} catch (e) {
  console.log("should not reach");
}

// try/catch with throw
try {
  throw "error message";
} catch (e) {
  console.log(e);
}

// try/catch with numeric throw
try {
  throw 42;
} catch (e) {
  console.log("caught " + e);
}

// try/catch with object
try {
  throw { msg: "oops" };
} catch (e) {
  console.log("caught object");
}
