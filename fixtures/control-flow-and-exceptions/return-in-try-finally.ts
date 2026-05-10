// Return inside try with finally: finally must run before return takes effect
function testReturnFinally() {
  try {
    return "from-try";
  } finally {
    console.log("finally");
  }
}

// If finally overrides with its own return, it takes precedence
function testFinallyOverridesReturn() {
  try {
    return 1;
  } finally {
    return 2;
  }
}

// Finally overrides throw with return
function testFinallyOverridesThrow() {
  try {
    throw "error";
  } finally {
    return "overridden";
  }
}

console.log(testReturnFinally());
console.log(testFinallyOverridesReturn());
console.log(testFinallyOverridesThrow());
