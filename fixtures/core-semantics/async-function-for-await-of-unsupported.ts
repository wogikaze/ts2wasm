// Diagnostic fixture for the async function wrapper used by for-await-of tests.
async function f() {
  for await (var value of values) {
    console.log(value);
  }
}
