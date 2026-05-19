let source = 'throw { name: "Error", message: "indirect boom", x: 5, cb: function add(a) { return this.x + a; } }';

try {
  (0, eval)(source);
} catch (error) {
  console.log(error.name);
  console.log(error.cb(2));
}
