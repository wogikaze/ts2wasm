let box = {
  value: 7,
  read(delta) {
    let source = "this.value + ':' + arguments[0]";
    return eval(source);
  },
};

console.log(box.read(5));
