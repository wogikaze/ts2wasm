class Base {
  #baseValue = 10;
  baseRead() {
    return this.#baseValue;
  }
}

class Derived extends Base {
  #derivedValue = 20;
  derivedRead() {
    return this.#derivedValue;
  }
  combinedRead() {
    return this.#derivedValue + this.baseRead();
  }
}

let d = new Derived();
console.log(d.baseRead());
console.log(d.derivedRead());
console.log(d.combinedRead());
