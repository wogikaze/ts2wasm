class Counter {
  constructor(seed) {
    this.seed = seed;
  }

  value(delta) {
    return this.seed + delta;
  }
}

let counter = new Counter(4);
console.log(counter.value(3));
