// W5.6: Getter/setter via class syntax (build_smoke only)
// Object.defineProperty has a known backend bug, so test via class get/set

class Counter {
  private _val: number = 0;

  get value(): number {
    return this._val;
  }

  set value(v: number) {
    this._val = v;
  }

  inc(): void {
    this._val = this._val + 1;
  }
}

const c = new Counter();
console.log(c.value);
c.value = 5;
console.log(c.value);
c.inc();
console.log(c.value);
