class Box {
  constructor(value) {
    this.value = value;
    let source = "this.value + ':' + arguments[0]";
    console.log(eval(source));
  }
}

new Box(7);
