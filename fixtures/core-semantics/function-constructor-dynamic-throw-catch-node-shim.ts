let body = "throw new Error('function boom')";
let fn = Function(body);

try {
  fn();
} catch (err) {
  console.log(err.name);
  console.log(err.message);
}
