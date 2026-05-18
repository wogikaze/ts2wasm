class Box {
  read(value) {
    return eval("arguments[0]");
  }
}

let box = new Box();
console.log(box.read(9));
