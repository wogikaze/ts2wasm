// Unlabeled break inside try-finally: finally must run before break takes effect
function testBreakInTryFinally(): number {
  let result = 0;
  for (let i = 0; i < 3; i++) {
    try {
      result = result + 1;
      break;
    } finally {
      result = result + 10;
    }
  }
  return result;
}

// Unlabeled continue inside try-finally: finally must run before continue takes effect
function testContinueInTryFinally(): number {
  let result = 0;
  for (let i = 0; i < 3; i++) {
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

// Labeled break inside try-finally targeting the containing loop
function testLabeledBreakInTryFinally(): number {
  let result = 0;
  myLoop:
  for (let i = 0; i < 3; i++) {
    try {
      result = result + 1;
      break myLoop;
    } finally {
      result = result + 10;
    }
  }
  return result;
}

// Labeled continue inside try-finally targeting the containing loop
function testLabeledContinueInTryFinally(): number {
  let result = 0;
  myLoop:
  for (let i = 0; i < 3; i++) {
    try {
      if (i === 1) {
        continue myLoop;
      }
      result = result + 1;
    } finally {
      result = result + 10;
    }
  }
  return result;
}

console.log(testBreakInTryFinally());
console.log(testContinueInTryFinally());
console.log(testLabeledBreakInTryFinally());
console.log(testLabeledContinueInTryFinally());
