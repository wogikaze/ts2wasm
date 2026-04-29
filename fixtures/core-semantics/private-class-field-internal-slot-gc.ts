class Box {
  #value = "private-root";

  read() {
    return this.#value;
  }
}

let box = new Box();
let i = 0;
let s = "";

while (i < 2500) {
  s = "private-pressure-" + i;
  i = i + 1;
}

console.log(box.read());
