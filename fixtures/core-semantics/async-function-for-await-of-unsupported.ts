// Diagnostic fixture for the async function wrapper used by for-await-of tests.
async function f() {
  let values = [];
  for await (var value of values) {
    console.log(value);
  }
}
