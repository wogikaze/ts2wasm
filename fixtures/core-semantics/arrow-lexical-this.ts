class Counter {
  constructor(seed) {
    this.seed = seed;
  }

  add(delta) {
    const addSeed = () => this.seed + delta;
    return addSeed();
  }
}

let counter = new Counter(8);
console.log(counter.add(5));
