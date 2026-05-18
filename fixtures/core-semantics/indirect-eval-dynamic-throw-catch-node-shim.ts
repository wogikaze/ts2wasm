let source = "throw new Error('indirect boom')";

try {
  globalThis.eval(source);
} catch (err) {
  console.log(err.name);
  console.log(err.message);
}
