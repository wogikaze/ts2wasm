// Iterator protocol test: for-of with break (avoids multi-call iteration)
// Tests that GetIterator and IteratorNext work for a single-element case.
const iterable = {
  [Symbol.iterator]: function () {
    return {
      next: function () {
        return { value: 42, done: false };
      }
    };
  }
};
for (const val of iterable) {
  console.log(val);
  break;
}
