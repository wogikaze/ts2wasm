// return inside try with finally - finally must run before return takes effect
function testReturnFinally(): string {
  try {
    return "from-try";
  } finally {
    console.log("finally runs before return");
  }
}
console.log("result: " + testReturnFinally());

// if finally overrides with its own return
function testFinallyOverridesReturn(): number {
  try {
    return 1;
  } finally {
    return 2;
  }
}
console.log("override: " + testFinallyOverridesReturn());

// finally overrides throw with return
function testFinallyOverridesThrow(): string {
  try {
    throw "error";
  } finally {
    return "overridden";
  }
}
console.log("overridden: " + testFinallyOverridesThrow());

// return in nested try/catch/finally
function testNestedReturn(): string {
  try {
    try {
      return "inner";
    } finally {
      console.log("inner finally");
    }
  } finally {
    console.log("outer finally");
  }
}
console.log("nested return: " + testNestedReturn());
