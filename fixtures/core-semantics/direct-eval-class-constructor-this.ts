class Box {
  constructor(seed) {
    eval("this.seed = seed");
  }

  read() {
    return this.seed;
  }
}

console.log(new Box(7).read());
