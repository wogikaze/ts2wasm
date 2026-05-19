var $262 = {};
let source = 'throw { name: "Error", message: "test262 boom", x: 5, cb: function add(a) { return this.x + a; } }';

try {
  $262.evalScript(source);
} catch (error) {
  console.log(error.name);
  console.log(error.cb(2));
}
