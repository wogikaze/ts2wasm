function run() {
  let source = 'throw { name: "Error", message: "object boom", x: 5, cb: function add(a) { return this.x + a; } }';
  try {
    eval(source);
  } catch (error) {
    console.log(error.name);
    console.log(error.cb(2));
  }
}

run();
