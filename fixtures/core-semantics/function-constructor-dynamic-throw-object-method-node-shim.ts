let body = 'throw { name: "Error", message: "function boom", x: 5, cb: function add(a) { return this.x + a; } }';
let fn = Function(body);

try {
  fn();
} catch (error) {
  console.log(error.name);
  console.log(error.cb(2));
}
