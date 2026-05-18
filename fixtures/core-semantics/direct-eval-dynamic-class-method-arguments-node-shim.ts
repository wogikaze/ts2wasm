class Box {
  read(value) {
    let source = "arguments[0] + ':' + arguments.length";
    return eval(source);
  }
}

let box = new Box();
console.log(box.read(9));
