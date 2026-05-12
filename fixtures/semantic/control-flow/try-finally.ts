// basic try/finally (no catch, no throw)
try {
  console.log("try body");
} finally {
  console.log("finally body");
}

// try/finally with throw: finally runs before propagation
// Wrap to prevent uncaught exception from terminating the test
try {
  try {
    throw "err";
  } finally {
    console.log("finally after throw");
  }
} catch (e) {
  console.log("caught after finally: " + e);
}

// try/finally with variable mutation in finally
let x = 1;
try {
  x = x + 1;
} finally {
  x = x + 10;
}
console.log("x after try/finally " + x);
