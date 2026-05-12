class Box {
  constructor(value) {
    this.value = value;
  }

  read() {
    return this.value;
  }
}

function makeBox() {
  return new Box(7);
}

console.log(makeBox().read());
