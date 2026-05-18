class Box {
  constructor(value) {
    this.value = value;
  }

  read(delta) {
    let source = "this.value + ':' + arguments[0]";
    let run = () => eval(source);
    return run();
  }
}

let box = new Box(7);
console.log(box.read(5));
