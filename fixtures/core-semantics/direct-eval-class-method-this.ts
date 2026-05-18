class Box {
  constructor(seed) {
    this.seed = seed;
  }

  read() {
    return eval("this.seed");
  }
}

console.log(new Box(5).read());
