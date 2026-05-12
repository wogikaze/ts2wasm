// break inside try/finally: finally must run before break
function testBreakInTryFinally(): number {
  let result = 0;
  for (let i = 0; i < 3; i = i + 1) {
    try {
      result = result + 1;
      break;
    } finally {
      result = result + 10;
    }
  }
  return result;
}
console.log("break-try-finally: " + testBreakInTryFinally());

// continue inside try/finally
function testContinueInTryFinally(): number {
  let result = 0;
  for (let i = 0; i < 3; i = i + 1) {
    try {
      if (i === 1) {
        continue;
      }
      result = result + 1;
    } finally {
      result = result + 10;
    }
  }
  return result;
}
console.log("continue-try-finally: " + testContinueInTryFinally());

// labeled break inside try/finally
function testLabeledBreakInTryFinally(): number {
  let result = 0;
  myLoop:
  for (let i = 0; i < 3; i = i + 1) {
    try {
      result = result + 1;
      break myLoop;
    } finally {
      result = result + 10;
    }
  }
  return result;
}
console.log("labeled-break-try-finally: " + testLabeledBreakInTryFinally());
