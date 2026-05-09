// Custom iterator with Symbol.iterator (W5)
const iterable = {
  items: [10, 20, 30],
  [Symbol.iterator]() {
    let i = 0;
    const items = this.items;
    return {
      next() {
        if (i < items.length) {
          return { value: items[i++], done: false };
        }
        return { value: undefined, done: true };
      }
    };
  }
};

for (const val of iterable) {
  console.log(val);
}
