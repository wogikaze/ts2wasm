function read(delta) {
  return this.seed + delta;
}

let first = { seed: 4, read: read };
let second = { seed: 9, read: read };

console.log(first.read(3));
console.log(second.read(3));
