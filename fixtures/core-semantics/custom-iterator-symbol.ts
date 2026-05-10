// Custom iterable with Symbol.iterator — iterator protocol for-of test
const iterable = {
  [Symbol.iterator]: function () {
    const items = [10, 20, 30];
    const state = { i: 0 };
    return {
      next: function () {
        const idx = state.i;
        state.i = state.i + 1;
        if (idx < items.length) {
          return { value: items[idx], done: false };
        }
        return { value: undefined, done: true };
      }
    };
  }
};
for (const val of iterable) {
  console.log(val);
}
