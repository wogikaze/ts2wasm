// Test prototype getter/setter in class body
class Counter {
  _count: number = 0;

  get count() {
    return this._count;
  }

  set count(val: number) {
    this._count = val;
  }

  increment() {
    this._count = this._count + 1;
  }
}

const c = new Counter();
c.count = 10;
c.increment();
console.log(c.count);
