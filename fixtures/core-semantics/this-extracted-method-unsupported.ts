class Box {
  constructor(value) {
    this.value = value;
  }

  read() {
    return this.value;
  }
}

let box = new Box(7);
let read = box.read;

console.log(read());
