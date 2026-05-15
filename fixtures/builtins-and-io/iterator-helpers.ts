// Iterator helpers (issue I-20260513-538Q92)
// Minimal build_smoke fixture for Iterator.from and Iterator.prototype methods
// Reference: ES2024+ Iterator helpers proposal

function makeIter(arr: number[]) {
  let i = 0;
  return {
    [Symbol.iterator]() { return this; },
    next() {
      if (i < arr.length) {
        return { value: arr[i++], done: false };
      }
      return { value: undefined, done: true };
    }
  };
}

// Basic iteration with for-of works
let items: number[] = [];
for (let x of makeIter([1, 2, 3])) {
  items.push(x);
}
console.log(items.length === 3 ? "pass" : "fail");
