Set.prototype.originalAdd = Set.prototype.add;

function patchedAdd(value) {
  if (this.counter) {
    this.counter = this.counter + 1;
  } else {
    this.counter = 1;
  }
  Set.prototype.originalAdd.call(this, value);
}

Set.prototype.add = patchedAdd;

let s = new Set([1, 2]);
console.log(s.counter);
console.log(s.has(1));
console.log(s.has(2));
