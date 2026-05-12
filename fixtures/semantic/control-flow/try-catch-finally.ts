// try/catch/finally: no error
try {
  console.log("try no error");
} catch (e) {
  console.log("catch should not run");
} finally {
  console.log("finally no error");
}

// try/catch/finally: with error
try {
  throw "caught";
} catch (e) {
  console.log("caught: " + e);
} finally {
  console.log("finally after catch");
}

// try/catch/finally: nested
let results = "";
try {
  try {
    throw "inner";
  } catch (e) {
    results = results + "inner-caught,";
    throw "outer";
  } finally {
    results = results + "inner-finally,";
  }
} catch (e) {
  results = results + "outer-caught,";
} finally {
  results = results + "outer-finally,";
}
console.log("nested: " + results);
