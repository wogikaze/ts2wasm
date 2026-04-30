class Counter {
  #value = 1;

  readFrom(other) {
    return other.#value;
  }

  writeTo(other, next) {
    other.#value = next;
    return other.#value;
  }
}

let first = new Counter();
let second = new Counter();

console.log(first.readFrom(second));
console.log(first.writeTo(second, 5));
console.log(second.readFrom(first));
console.log(second.readFrom(second));
