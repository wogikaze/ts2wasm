// Private fields surviving GC pressure.
// This tests that private slots are correctly scanned by the GC mark loop,
// so private field values that reference heap objects survive allocation pressure.

class Holder {
  #value: string;

  constructor(v: string) {
    this.#value = v;
  }

  read(): string {
    return this.#value;
  }
}

let obj = new Holder("gc-root-value");
let i = 0;
let s = "";

// Allocation pressure to trigger GC
while (i < 2500) {
  s = "pressure-" + i;
  i = i + 1;
}

console.log(obj.read());
